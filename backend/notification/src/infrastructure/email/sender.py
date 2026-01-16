from __future__ import annotations

import logging
import smtplib
from email.message import EmailMessage

from domain.models import Notification

logger = logging.getLogger(__name__)


class SmtpEmailSender:
    def __init__(
        self,
        host: str,
        port: int,
        username: str | None,
        password: str | None,
        sender_email: str,
        use_tls: bool = True,
    ) -> None:
        self._host = host
        self._port = port
        self._username = username
        self._password = password
        self._sender_email = sender_email
        self._use_tls = use_tls

    def send(self, notification: Notification) -> None:
        if not self._host or not self._sender_email:
            raise RuntimeError("SMTP configuration is missing")
        recipient_email = notification.data.get("email")
        if not recipient_email:
            raise ValueError("notification missing recipient email")
        message = EmailMessage()
        message["From"] = self._sender_email
        message["To"] = recipient_email
        message["Subject"] = notification.template or "Notification"
        message.set_content(self._render_body(notification))
        logger.info("sending email notification %s to %s", notification.notification_id, recipient_email)
        with smtplib.SMTP(self._host, self._port, timeout=10) as client:
            if self._use_tls:
                client.starttls()
            if self._username and self._password:
                client.login(self._username, self._password)
            client.send_message(message)

    @staticmethod
    def _render_body(notification: Notification) -> str:
        payload_lines = [f"{key}: {value}" for key, value in notification.data.items()]
        payload = "\n".join(payload_lines)
        return f"{notification.template}\n\n{payload}".strip()
