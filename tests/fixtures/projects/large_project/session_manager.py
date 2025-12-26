from dataclasses import dataclass, field
from typing import Dict, List, Optional, Any
from datetime import datetime, timedelta
import hashlib
import json
import uuid


@dataclass
class User:
    id: str
    username: str
    email: str
    created_at: datetime
    roles: List[str] = field(default_factory=list)
    preferences: Dict[str, Any] = field(default_factory=dict)


@dataclass
class Session:
    user_id: str
    token: str
    created_at: datetime
    expires_at: datetime
    last_activity: datetime


class SessionManager:
    def __init__(self, session_duration_hours: int = 24):
        self.sessions: Dict[str, Session] = {}
        self.users: Dict[str, User] = {}
        self.session_duration = timedelta(hours=session_duration_hours)

    def register_user(self, username: str, email: str, roles: Optional[List[str]] = None) -> str:
        user_id = str(uuid.uuid4())
        user = User(
            id=user_id,
            username=username,
            email=email,
            created_at=datetime.now(),
            roles=roles or ["user"],
        )
        self.users[user_id] = user
        return user_id

    def create_session(self, user_id: str) -> str:
        if user_id not in self.users:
            raise ValueError("User not found")

        token = hashlib.sha256(f"{user_id}{datetime.now().isoformat()}".encode()).hexdigest()
        now = datetime.now()
        session = Session(
            user_id=user_id,
            token=token,
            created_at=now,
            expires_at=now + self.session_duration,
            last_activity=now,
        )
        self.sessions[token] = session
        return token

    def validate_session(self, token: str) -> Optional[User]:
        if token not in self.sessions:
            return None

        session = self.sessions[token]
        if session.expires_at < datetime.now():
            del self.sessions[token]
            return None

        session.last_activity = datetime.now()
        return self.users.get(session.user_id)

    def revoke_session(self, token: str) -> bool:
        if token in self.sessions:
            del self.sessions[token]
            return True
        return False

    def cleanup_expired_sessions(self) -> int:
        now = datetime.now()
        expired = [t for t, s in self.sessions.items() if s.expires_at < now]
        for t in expired:
            del self.sessions[t]
        return len(expired)


if __name__ == "__main__":
    manager = SessionManager()

    user_id = manager.register_user(
        "alice",
        "alice@example.com",
        roles=["user", "admin"]
    )
    print(f"Registered user: {user_id}")

    token = manager.create_session(user_id)
    print(f"Session token: {token}")

    user = manager.validate_session(token)
    if user:
        print(f"Session valid for: {user.username}")

    expired_count = manager.cleanup_expired_sessions()
    print(f"Cleaned up {expired_count} expired sessions")
