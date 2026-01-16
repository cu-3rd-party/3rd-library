import grpc

from application.service import ContentService
from proto import content_pb2, content_pb2_grpc


class ContentGrpcService(content_pb2_grpc.ContentServiceServicer):
    def __init__(self, service: ContentService) -> None:
        self._service = service

    def Upload(self, request: content_pb2.UploadRequest, context: grpc.ServicerContext) -> content_pb2.UploadResponse:
        try:
            item = self._service.upload(
                owner_id=request.owner_id,
                title=request.title,
                description=request.description,
                filename=request.filename,
                file_bytes=request.file_bytes,
            )
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))
        response = content_pb2.UploadResponse(content=_to_proto(item))
        return response

    def Get(self, request: content_pb2.GetRequest, context: grpc.ServicerContext) -> content_pb2.GetResponse:
        try:
            item, data = self._service.get(request.content_id)
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))
        except KeyError as exc:
            context.abort(grpc.StatusCode.NOT_FOUND, str(exc))
        return content_pb2.GetResponse(content=_to_proto(item), file_bytes=data)

    def List(self, request: content_pb2.ListRequest, context: grpc.ServicerContext) -> content_pb2.ListResponse:
        try:
            items, next_token = self._service.list_items(request.page_size, request.page_token)
        except ValueError as exc:
            context.abort(grpc.StatusCode.INVALID_ARGUMENT, str(exc))

        return content_pb2.ListResponse(items=[_to_proto(item) for item in items], next_page_token=next_token)


def _to_proto(item: "ContentItem") -> content_pb2.Content:
    return content_pb2.Content(
        id=item.id,
        owner_id=item.owner_id,
        title=item.title,
        description=item.description,
        filename=item.filename,
        size_bytes=item.size_bytes,
        created_at=item.created_at,
    )
