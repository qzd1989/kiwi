from __future__ import annotations
from dataclasses import dataclass
from .point import Point
from types import SimpleNamespace
from typing import Optional


@dataclass(kw_only=True)
class WeightPoint:
    point: Point
    weight: float

    def to_dict(self) -> dict:
        return {"point": self.point, "weight": self.weight}

    @staticmethod
    def from_namespace(ns: SimpleNamespace | None) -> Optional[WeightPoint]:
        if ns is None:
            return None
        return WeightPoint(point=Point.from_namespace(ns.point), weight=ns.weight)

    @staticmethod
    def from_namespace_array(
        ns_array: list[SimpleNamespace] | None,
    ) -> list[WeightPoint]:
        if ns_array is None:
            return []
        return [WeightPoint.from_namespace(ns) for ns in ns_array if ns is not None]
