from fastapi import FastAPI, UploadFile, File
from nudenet import NudeDetector
import uvicorn
import shutil
import os

app = FastAPI()
detector = NudeDetector()

@app.post("/infer")
async def infer(f1: UploadFile = File(...)):
    temp_path = f"temp_{f1.filename}"
    with open(temp_path, "wb") as buffer:
        shutil.copyfileobj(f1.file, buffer)

    try:
        detections = detector.detect(temp_path)
        
        formatted_predictions = []
        for d in detections:
            formatted_predictions.append({
                "class": d['class'],
                "score": float(d['score']),
                "box": d['box']  # [x, y, w, h]
            })
            
        return {"prediction": [formatted_predictions], "success": True}
    
    finally:
        if os.path.exists(temp_path):
            os.remove(temp_path)

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8080)
