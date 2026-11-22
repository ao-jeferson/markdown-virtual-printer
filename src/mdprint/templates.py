"""Small built-in templates for chat and report exports."""

from datetime import date


def render_report(content: str, title: str = "Relatório", report_date: date | None = None) -> str:
    day = report_date or date.today()
    body = content.strip()
    return f"# {title.strip()}\n\n_Data: {day.isoformat()}_\n\n{body}\n"
