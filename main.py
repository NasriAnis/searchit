import uvicorn

if __name__ == "__main__":
    uvicorn.run("api.main:app", port=8000, log_level="info")