FROM python:3.14-slim
WORKDIR /app
COPY pyproject.toml uv.lock ./
RUN pip install --no-cache-dir uv && uv sync --frozen
COPY src ./src
EXPOSE 50054
CMD ["uv", "run", "python", "src/main.py"]
