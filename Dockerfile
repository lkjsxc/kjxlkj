FROM python:3.12-alpine

WORKDIR /workspace

COPY docs ./docs

EXPOSE 8080

HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=5 \
  CMD python -c "import urllib.request; urllib.request.urlopen('http://127.0.0.1:8080/docs/README.md', timeout=2)"

CMD ["python", "-m", "http.server", "8080", "--directory", "/workspace"]
