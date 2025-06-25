from __future__ import annotations
from dataclasses import dataclass
from types import SimpleNamespace
from typing import Optional


@dataclass(kw_only=True)
class Point:
    x: int
    y: int

    def to_dict(self) -> dict:
        return {"x": self.x, "y": self.y}

    @staticmethod
    def from_namespace(ns: SimpleNamespace | None) -> Optional[Point]:
        if ns is None:
            return None
        return Point(x=ns.x, y=ns.y)
