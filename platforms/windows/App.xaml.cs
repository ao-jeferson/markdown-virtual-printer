using Windows.ApplicationModel.Activation;
using Windows.UI.Xaml;
using Windows.UI.Xaml.Controls;

namespace MarkdownVirtualPrinter
{
    sealed partial class App : Application
    {
        protected override void OnLaunched(LaunchActivatedEventArgs args)
        {
            Window.Current.Content = new Frame
            {
                Content = new TextBlock
                {
                    Text = "Markdown Virtual Printer is installed. Select it in an application's print dialog.",
                    TextWrapping = TextWrapping.Wrap,
                    Margin = new Thickness(24)
                }
            };
            Window.Current.Activate();
        }
    }
}
