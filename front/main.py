from fu import json
import os
from os.path import *
import re

cur = dirname(__file__)
root = dirname(cur)
"""

# 获取全部题目
leetcode是前后端分离的，在leetcode题目列表页可以找到问题列表的接口。 
https://leetcode-cn.com/api/problems/all/
 

"""
all = json.load(join(cur, 'all.json'))
trans = json.load(join(cur, 'problems.json'))
ma = {}
for i in trans:
    ma[str(i['questionId'])] = i['title']
for i in all:
    i['chinese'] = ma.get(str(i['stat']['frontend_question_id']))
all.reverse()


#######Rust


########绑定Rust
def get_common_dir(folder: str, suffix: str):
    files = os.listdir(folder)
    files = [i for i in files if i.endswith(suffix)]
    ma = {}
    for file in files:
        filepath = join(folder, file)
        res = re.search("\d+", file)
        if not res: continue
        pid = int(res.group())
        ma[str(pid)] = open(filepath).read()
    return ma


def get_rust():
    return get_common_dir(join(root, "rust/src/bin"), ".rs")


def get_java():
    return get_common_dir(join(root, "java/src/leetcode"), '.java')


def get_python():
    return get_common_dir(join(root, "python"), ".py")


def get_cpp():
    return get_common_dir(join(root, "cpp"), ".cpp")


def get_js():
    return get_common_dir(join(root, "js"), ".js")


def get_go():
    return get_common_dir(join(root, "go"), ".go")


lang = {
    'rust': get_rust(),
    'java': get_java(),
    'python': get_python(),
    'go': get_go(),
    "cpp": get_cpp(),
}
for i in all:
    answer = {}
    for k, v in lang.items():
        pid = i['stat']['frontend_question_id']
        answer[k] = v.get(str(pid))
    i['answer'] = answer
json.dump(all, join(cur, 'index.json'), indent=2)
