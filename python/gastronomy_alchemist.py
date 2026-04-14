import requests
import os
from typing import List, Optional, Dict
import json

class DaithonGastronomyAlchemist:
    """
    Módulo de Gastronomía Alquímica para Daithon.
    Kukuku... La cocina no es más que química comestible.
    """
    
    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key or os.getenv("SPOONACULAR_API_KEY")
        self.base_url = "https://api.spoonacular.com"
        self.is_configured = self.api_key is not None

    def get_headers(self):
        return {
            "Content-Type": "application/json",
            "x-api-key": self.api_key
        }

    def search_recipes_by_ingredients(self, ingredients: str, number: int = 5) -> Dict:
        """Busca recetas basadas en ingredientes. Kukuku... transmutación pura."""
        if not self.is_configured:
            return {"error": "Spoonacular API no configurada. Ineficiente."}
            
        url = f"{self.base_url}/recipes/findByIngredients"
        params = {
            "ingredients": ingredients,
            "number": number,
            "ranking": 1,
            "ignorePantry": True
        }
        
        try:
            response = requests.get(url, params=params, headers=self.get_headers())
            response.raise_for_status()
            return response.json()
        except Exception as e:
            return {"error": str(e)}

    def get_recipe_details(self, recipe_id: int) -> Dict:
        """Obtiene el proceso alquímico completo de una receta."""
        if not self.is_configured:
            return {"error": "API Key faltante."}
            
        url = f"{self.base_url}/recipes/{recipe_id}/information"
        try:
            response = requests.get(url, headers=self.get_headers())
            response.raise_for_status()
            return response.json()
        except Exception as e:
            return {"error": str(e)}

    def analyze_food_science(self, query: str):
        """
        Daithon analiza la ciencia detrás de un alimento.
        Añade humor científico y terminología técnica.
        """
        # Aquí Daithon usaría su lógica interna (y no la de Ollama) 
        # para explicar por qué el colágeno es elegancia estructural, etc.
        pass

if __name__ == "__main__":
    # Test rápido de integración
    alchemist = DaithonGastronomyAlchemist()
    print("Módulo Alquímico de Daithon: Iniciado. Kukuku...")
