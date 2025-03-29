import requests
import os
import yaml
from datetime import datetime

templates_dir = os.path.dirname(os.path.abspath(__file__))
output_file = os.path.join(templates_dir, 'leetcode-problems.yaml')

def fetch_leetcode_problems():
    # LeetCode GraphQL API
    url = "https://leetcode.cn/graphql"
    
    # 查询所有题目的 GraphQL 请求
    query = """
    query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
        problemsetQuestionList(
            categorySlug: $categorySlug
            limit: $limit
            skip: $skip
            filters: $filters
        ) {
            hasMore
            total
            questions {
                titleCn
                titleSlug
                title
                difficulty
                frontendQuestionId
            }
        }
    }
    """
    
    variables = {
        "categorySlug": "",
        "skip": 0,
        "limit": 100,  # 每页100题
        "filters": {}
    }
    
    headers = {
        "Content-Type": "application/json",
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
    }
    
    problems = []
    while True:
        response = requests.post(url, json={
            "query": query,
            "variables": variables
        }, headers=headers)
        
        data = response.json()
        question_list = data['data']['problemsetQuestionList']
        
        # 添加本页题目
        problems.extend(question_list['questions'])
        
        # 检查是否还有更多题目
        if not question_list['hasMore']:
            break
            
        variables['skip'] += variables['limit']
    
    return problems

def generate_yaml():
    problems = fetch_leetcode_problems()
    
    # 构建 YAML 数据结构
    yaml_data = {
        'name': 'leetcode-problem-set',
        'date': datetime.now().strftime('%Y-%m-%d'),
        'problem-set': []
    }
    
    # 添加题目信息
    for problem in problems:
        problem_data = {
            'id': problem['frontendQuestionId'],
            'url': f"https://leetcode.cn/problems/{problem['titleSlug']}"
        }
        yaml_data['problem-set'].append(problem_data)
    
    # 将数据写入 YAML 文件
    with open(output_file, 'w', encoding='utf-8') as f:
        yaml.safe_dump(yaml_data, f, allow_unicode=True, sort_keys=False)

if __name__ == '__main__':
    generate_yaml()