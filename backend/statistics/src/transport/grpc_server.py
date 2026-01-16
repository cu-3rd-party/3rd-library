from __future__ import annotations

import grpc

from application.service import StatisticsService
from domain.models import InteractionType
from proto import statistics_pb2, statistics_pb2_grpc


class StatisticsGrpcService(statistics_pb2_grpc.StatisticsServiceServicer):
    def __init__(self, service: StatisticsService) -> None:
        self._service = service

    def RecordInteraction(
        self,
        request: statistics_pb2.RecordInteractionRequest,
        context: grpc.ServicerContext,
    ) -> statistics_pb2.RecordInteractionResponse:
        try:
            interaction_type = _map_interaction_type(request.type)
            recorded_id = self._service.record(
                content_id=request.content_id,
                user_id=request.user_id,
                interaction_type=interaction_type,
                occurred_at=request.occurred_at,
            )
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))
        return statistics_pb2.RecordInteractionResponse(recorded_id=recorded_id)

    def GetContentStats(
        self,
        request: statistics_pb2.GetContentStatsRequest,
        context: grpc.ServicerContext,
    ) -> statistics_pb2.GetContentStatsResponse:
        try:
            stats = self._service.stats_for(request.content_id)
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))

        return statistics_pb2.GetContentStatsResponse(
            stats=statistics_pb2.ContentStats(
                views=stats.views,
                downloads=stats.downloads,
                unique_users=stats.unique_users,
                last_interaction_at=stats.last_interaction_at,
            )
        )


def _map_interaction_type(pb_type: int) -> InteractionType:
    if pb_type == statistics_pb2.INTERACTION_TYPE_VIEW:
        return InteractionType.VIEW
    if pb_type == statistics_pb2.INTERACTION_TYPE_DOWNLOAD:
        return InteractionType.DOWNLOAD
    raise ValueError("unsupported interaction type")
