FROM golang:1.25 as builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY cmd ./cmd
COPY internal ./internal
RUN CGO_ENABLED=0 GOOS=linux go build -o /auth ./cmd/auth

FROM gcr.io/distroless/base-debian12
COPY --from=builder /auth /auth
EXPOSE 50051
ENTRYPOINT ["/auth"]
