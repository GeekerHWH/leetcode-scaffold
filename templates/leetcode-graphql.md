# GraphQL 参数说明
```python
# https://github.com/akarsh1995/leetcode-graphql-queries/blob/main/problemset_page/problemset_page.graphql#L50

variables = {
    "categorySlug": "",    # 题目分类，例如："algorithms"（算法）, "database"（数据库）, "shell"（终端）等
                          # 空字符串表示不进行分类筛选
    
    "skip": 0,            # 跳过前面多少条数据，用于分页
                          # 比如 skip=100 就是从第101题开始
    
    "limit": 100,         # 每次请求返回的题目数量
                          # 比如 limit=100 就是每页显示100道题
    
    "filters": {          # 筛选条件，可以包含多个筛选项
        "difficulty": "",     # 难度: "EASY", "MEDIUM", "HARD"
        "status": "",        # 完成状态: "NOT_STARTED", "AC", "TRIED"
        "tags": [],         # 标签: ["数组", "字符串", "动态规划"]等
        "searchKeywords": "" # 搜索关键词
    }
}
```
示例用法：
```python
# 示例1：获取所有简单难度的题目
variables = {
    "categorySlug": "",
    "skip": 0,
    "limit": 100,
    "filters": {
        "difficulty": "EASY"
    }
}

# 示例2：获取包含"动态规划"标签的题目
variables = {
    "categorySlug": "",
    "skip": 0,
    "limit": 100,
    "filters": {
        "tags": ["动态规划"]
    }
}

# 示例3：分页获取第201-300题
variables = {
    "categorySlug": "",
    "skip": 200,
    "limit": 100,
    "filters": {}
}

```