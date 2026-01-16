from __future__ import annotations

import grpc

from application.service import EngagementService
from domain.models import InteractionType
from proto import engagement_pb2, engagement_pb2_grpc


class EngagementGrpcService(engagement_pb2_grpc.EngagementServiceServicer):
    def __init__(self, service: EngagementService) -> None:
        self._service = service

    def RecordInteraction(
        self,
        request: engagement_pb2.RecordInteractionRequest,
        context: grpc.ServicerContext,
    ) -> engagement_pb2.RecordInteractionResponse:
        try:
            interaction_type = _map_interaction_type(request.type)
            interaction_id = self._service.record(
                content_id=request.content_id,
                user_id=request.user_id,
                interaction_type=interaction_type,
                occurred_at=request.occurred_at,
            )
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))
        return engagement_pb2.RecordInteractionResponse(interaction_id=interaction_id)

    def GetEngagement(
        self,
        request: engagement_pb2.GetEngagementRequest,
        context: grpc.ServicerContext,
    ) -> engagement_pb2.GetEngagementResponse:
        try:
            summary = self._service.summary(request.content_id)
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))

        return engagement_pb2.GetEngagementResponse(
            views=summary.views,
            downloads=summary.downloads,
            unique_users=summary.unique_users,
        )


def _map_interaction_type(pb_type: int) -> InteractionType:
    if pb_type == engagement_pb2.INTERACTION_TYPE_VIEW:
        return InteractionType.VIEW
    if pb_type == engagement_pb2.INTERACTION_TYPE_DOWNLOAD:
        return InteractionType.DOWNLOAD
    raise ValueError("unsupported interaction type")
