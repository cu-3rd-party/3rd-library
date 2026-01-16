FROM python:3.14-slim
WORKDIR /app
COPY pyproject.toml uv.lock ./
RUN pip install --no-cache-dir uv && uv sync --frozen
COPY src ./src
ENV PYTHONPATH=/app/src:/app/src/proto
EXPOSE 50054
CMD ["uv", "run", "python", "src/main.py"]
