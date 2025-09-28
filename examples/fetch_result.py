#!/usr/bin/env python3
"""
Example Python script to fetch education board results
"""

import requests
import json
from typing import Dict, Any

class EduBoardAPI:
    def __init__(self, base_url: str = "http://localhost:3000"):
        self.base_url = base_url
    
    def fetch_result(self, exam: str, year: str, board: str, roll: str, reg: str) -> Dict[str, Any]:
        """
        Fetch examination result
        
        Args:
            exam: Examination type (e.g., 'ssc', 'hsc')
            year: Year (e.g., '2024')
            board: Board name (e.g., 'dhaka')
            roll: Roll number
            reg: Registration number
            
        Returns:
            Dict containing result data
        """
        params = {
            'exam': exam,
            'year': year,
            'board': board,
            'roll': roll,
            'reg': reg
        }
        
        try:
            response = requests.get(f"{self.base_url}/fetch", params=params)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            print(f"Error fetching result: {e}")
            return {}

def main():
    # Initialize API client
    api = EduBoardAPI()
    
    # Example: Fetch a result
    result = api.fetch_result(
        exam='ssc',
        year='2024',
        board='dhaka',
        roll='123456',
        reg='1234567890'
    )
    
    if result:
        print("Student Information:")
        print(f"  Name: {result.get('name', 'N/A')}")
        print(f"  Roll: {result.get('roll', 'N/A')}")
        print(f"  Registration: {result.get('reg', 'N/A')}")
        print(f"  Result: {result.get('result', 'N/A')}")
        print(f"  GPA: {result.get('gpa', 'N/A')}")
        
        if result.get('grades'):
            print("\nSubject Grades:")
            for grade in result['grades']:
                print(f"  {grade['subject']}: {grade['grade']}")
    else:
        print("No result found or error occurred")

if __name__ == "__main__":
    main()
