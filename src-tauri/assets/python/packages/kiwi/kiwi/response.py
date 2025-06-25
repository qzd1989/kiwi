from dataclasses import dataclass
from typing import Literal, Any, Optional


@dataclass(kw_only=True)
class Response:
    status: Literal["success", "error"]
    message: Optional[str]
    data: Optional[Any]

    def to_dict(self) -> dict:
        return {
            "status": self.status,
            "message": self.message,
            "data": self.data,
        }
