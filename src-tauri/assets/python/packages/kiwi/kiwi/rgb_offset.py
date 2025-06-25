from __future__ import annotations
from dataclasses import dataclass
from types import SimpleNamespace
from typing import Optional


@dataclass(kw_only=True)
class RgbOffset:
    r: int
    g: int
    b: int

    def to_dict(self) -> dict:
        return {"r": self.r, "g": self.g, "b": self.b}

    @staticmethod
    def from_namespace(ns: SimpleNamespace | None) -> Optional[RgbOffset]:
        if ns is None:
            return None
        return RgbOffset(r=ns.r, g=ns.g, b=ns.b)
