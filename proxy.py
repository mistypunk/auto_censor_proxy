import requests
from mitmproxy import http

def response(flow: http.HTTPFlow):
    content_type = flow.response.headers.get("Content-Type", "")
    
    if "image/jpeg" in content_type or "image/png" in content_type:
        original_data = flow.response.content
        
        try:
            res = requests.post(
                "http://localhost:3000/censor", 
                data=original_data,
                timeout=10
            )
            
            if res.status_code == 200:
                flow.response.content = res.content
                print(f"✅ [CENSOR] Imagem processada: {flow.request.url[:50]}...")
                
        except Exception as e:
            print(f"❌ [ERRO] Falha ao comunicar com Rust: {e}")
