import grpc

from application.service import EngagementService
from proto import engagement_pb2, engagement_pb2_grpc


class EngagementGrpcService(engagement_pb2_grpc.EngagementServiceServicer):
    def __init__(self, service: EngagementService) -> None:
        self._service = service

    def AddComment(
        self,
        request: engagement_pb2.AddCommentRequest,
        context: grpc.ServicerContext,
    ) -> engagement_pb2.AddCommentResponse:
        try:
            comment = self._service.add_comment(
                content_id=request.content_id,
                user_id=request.user_id,
                body=request.body,
                created_at=request.created_at,
            )
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))
        return engagement_pb2.AddCommentResponse(comment=_to_comment_pb(comment))

    def ListComments(
        self,
        request: engagement_pb2.ListCommentsRequest,
        context: grpc.ServicerContext,
    ) -> engagement_pb2.ListCommentsResponse:
        try:
            comments = self._service.list_comments(request.content_id)
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))
        return engagement_pb2.ListCommentsResponse(comments=[_to_comment_pb(comment) for comment in comments])

    def SetLike(
        self,
        request: engagement_pb2.SetLikeRequest,
        context: grpc.ServicerContext,
    ) -> engagement_pb2.SetLikeResponse:
        try:
            likes = self._service.set_like(
                content_id=request.content_id,
                user_id=request.user_id,
                liked=request.liked,
            )
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))
        return engagement_pb2.SetLikeResponse(likes=likes, liked=request.liked)

    def GetEngagement(
        self,
        request: engagement_pb2.GetEngagementRequest,
        context: grpc.ServicerContext,
    ) -> engagement_pb2.GetEngagementResponse:
        try:
            summary = self._service.summary(request.content_id, request.user_id)
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))

        return engagement_pb2.GetEngagementResponse(
            likes=summary.likes,
            comments=summary.comments,
            liked_by_user=summary.liked_by_user,
        )


def _to_comment_pb(comment) -> engagement_pb2.Comment:
    return engagement_pb2.Comment(
        id=comment.id,
        content_id=comment.content_id,
        user_id=comment.user_id,
        body=comment.body,
        created_at=comment.created_at,
    )
