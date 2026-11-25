using System;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using System.Xml.Linq;
using Windows.ApplicationModel.Background;
using Windows.Graphics.Printing.Workflow;
using Windows.Storage;
using Windows.Storage.Streams;

namespace MarkdownVirtualPrinter
{
    public sealed class PrintWorkflowTask : IBackgroundTask
    {
        private BackgroundTaskDeferral _deferral;

        public void Run(IBackgroundTaskInstance taskInstance)
        {
            _deferral = taskInstance.GetDeferral();
            var details = (PrintWorkflowVirtualPrinterTriggerDetails)taskInstance.TriggerDetails;
            details.VirtualPrinterSession.VirtualPrinterDataAvailable += OnDataAvailable;
            details.VirtualPrinterSession.Start();
        }

        private async void OnDataAvailable(PrintWorkflowVirtualPrinterSession sender,
            PrintWorkflowVirtualPrinterDataAvailableEventArgs args)
        {
            var status = PrintWorkflowSubmittedStatus.Failed;
            try
            {
                if (!string.Equals(args.SourceContent.ContentType, "application/oxps", StringComparison.OrdinalIgnoreCase))
                    throw new InvalidDataException("Expected an OXPS print job.");

                using (var pdf = new InMemoryRandomAccessStream())
                {
                    var converter = args.GetPdlConverter(PrintWorkflowPdlConversionType.XpsToPdf);
                    await converter.ConvertPdlAsync(args.GetJobPrintTicket(), args.SourceContent.GetInputStream(), pdf);
                    if (pdf.Size == 0 || pdf.Size > 256UL * 1024 * 1024)
                        throw new InvalidDataException("The print job is empty or exceeds the size limit.");

                    pdf.Seek(0);
                    using (var reader = new DataReader(pdf.GetInputStreamAt(0)))
                    {
                        await reader.LoadAsync((uint)pdf.Size);
                        var input = new byte[(int)pdf.Size];
                        reader.ReadBytes(input);
                        var mode = KeepFormatting(args) ? 0U : 1U;
                        var markdown = ConvertJob(input, mode);
                        var target = await args.GetTargetFileAsync();
                        await FileIO.WriteBytesAsync(target, markdown);
                    }
                }
                status = PrintWorkflowSubmittedStatus.Succeeded;
            }
            catch (Exception)
            {
                status = PrintWorkflowSubmittedStatus.Failed;
            }
            finally
            {
                args.CompleteJob(status);
                _deferral.Complete();
            }
        }

        private static bool KeepFormatting(PrintWorkflowVirtualPrinterDataAvailableEventArgs args)
        {
            var xml = args.GetJobPrintTicket().XmlNode.GetXml();
            var root = XDocument.Parse(xml).Root;
            if (root == null) return true;
            var feature = root.Descendants().FirstOrDefault(e =>
                IsVendorName(root, (string)e.Attribute("name"), "KeepFormatting"));
            var optionName = (string)feature?.Elements().FirstOrDefault(e => e.Name.LocalName == "Option")?.Attribute("name");
            return optionName == null || !IsVendorName(root, optionName, "Off");
        }

        private static bool IsVendorName(XElement root, string qualifiedName, string localName)
        {
            if (string.IsNullOrEmpty(qualifiedName)) return false;
            var parts = qualifiedName.Split(':');
            var name = parts[parts.Length - 1];
            if (name != localName) return false;
            if (parts.Length == 1) return true;
            return root.GetNamespaceOfPrefix(parts[0])?.NamespaceName ==
                "https://ao-jeferson.github.io/markdown-virtual-printer/print-schema";
        }

        private static byte[] ConvertJob(byte[] input, uint mode)
        {
            var mime = System.Text.Encoding.UTF8.GetBytes("application/pdf");
            var title = System.Text.Encoding.UTF8.GetBytes("Print job");
            var inputPin = GCHandle.Alloc(input, GCHandleType.Pinned);
            var mimePin = GCHandle.Alloc(mime, GCHandleType.Pinned);
            var titlePin = GCHandle.Alloc(title, GCHandleType.Pinned);
            try
            {
                var length = MdvpConvertJob(inputPin.AddrOfPinnedObject(), new UIntPtr((uint)input.Length),
                    mimePin.AddrOfPinnedObject(), new UIntPtr((uint)mime.Length),
                    titlePin.AddrOfPinnedObject(), new UIntPtr((uint)title.Length), mode, IntPtr.Zero, UIntPtr.Zero);
                if (length <= 0 || length > 268435456) throw new InvalidDataException("Markdown generation failed.");
                var output = Marshal.AllocHGlobal((int)length);
                try
                {
                    var written = MdvpConvertJob(inputPin.AddrOfPinnedObject(), new UIntPtr((uint)input.Length),
                        mimePin.AddrOfPinnedObject(), new UIntPtr((uint)mime.Length),
                        titlePin.AddrOfPinnedObject(), new UIntPtr((uint)title.Length), mode, output, new UIntPtr((ulong)length));
                    if (written != length) throw new InvalidDataException("Markdown generation returned an invalid length.");
                    var result = new byte[(int)length];
                    Marshal.Copy(output, result, 0, result.Length);
                    return result;
                }
                finally { Marshal.FreeHGlobal(output); }
            }
            finally
            {
                inputPin.Free(); mimePin.Free(); titlePin.Free();
            }
        }

        [DllImport("printer_core.dll", EntryPoint = "mdvp_convert_job", CallingConvention = CallingConvention.Cdecl)]
        private static extern long MdvpConvertJob(IntPtr input, UIntPtr inputLength,
            IntPtr contentType, UIntPtr contentTypeLength, IntPtr title, UIntPtr titleLength,
            uint mode, IntPtr output, UIntPtr outputCapacity);
    }
}
