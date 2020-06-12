import numpy as np


class Node:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None


# np.random.seed(5)


def generate_tree():
    # 随机产生一个二叉树

    def insert(node: Node, value):
        if not node: return Node(value)
        if value > node.value:
            node.right = insert(node.right, value)
        else:
            node.left = insert(node.left, value)
        return node

    a = np.unique(np.random.randint(0, 30, 7))
    now = Node(15)
    np.random.shuffle(a)
    for i in a:
        insert(now, i)
    return now


def print_tree(node: Node):
    def go(node: Node, padding: int):
        if not node: return
        print(' ' * padding, node.value)
        go(node.left, padding + 1)
        go(node.right, padding + 1)

    go(node, 0)


def pre_order(node: Node):
    sta = [node]
    while sta:
        now = sta.pop()
        print(now.value, end=' ')
        now.right and sta.append(now.right)
        now.left and sta.append(now.left)


def mid_order(node: Node):
    sta = [(node, 0)]
    while sta:
        now, visited = sta.pop()
        if visited:
            print(now.value, end=',')
        else:
            now.right and sta.append((now.right, 0))
            sta.append((now, 1))
            now.left and sta.append((now.left, 0))


def back_order(node: Node):
    sta = [(node, 0)]
    while sta:
        now, visited = sta.pop()
        if visited:
            print(now.value, end=',')
        else:
            sta.append((now, 1))
            now.right and sta.append((now.right, 0))
            now.left and sta.append((now.left, 0))


class Recursive:
    @staticmethod
    def mid_order(x: Node):
        if x is None: return
        Recursive.mid_order(x.left)
        print(x.value, end=',')
        Recursive.mid_order(x.right)

    @staticmethod
    def pre_order(x: Node):
        if x is None: return
        print(x.value, end=',')
        Recursive.pre_order(x.left)
        Recursive.pre_order(x.right)

    @staticmethod
    def back_order(x: Node):
        if x is None:
            return
        Recursive.back_order(x.left)
        Recursive.back_order(x.right)
        print(x.value, end=',')


class Loop:
    """
    正经的非递归二叉树遍历写法
    """

    @staticmethod
    def pre_order(root: Node):
        sta = [root]
        while sta:
            now = sta.pop()
            print(now.value, end=',')
            now.right and sta.append(now.right)
            now.left and sta.append(now.left)

    @staticmethod
    def back_order(root):
        sta = [root]
        last = root
        while sta:
            now = sta[-1]
            if not now.left and not now.right or \
                    last == now.left and not now.right or last == now.right:
                print(now.value, end=',')
                last = now
                sta.pop()
            else:
                now.right and sta.append(now.right)
                now.left and sta.append(now.left)

    @staticmethod
    def mid_order(root: Node):
        sta = []
        while sta or root:
            while root:
                sta.append(root)
                root = root.left
            if sta:
                root = sta.pop()
                print(root.value, end=',')
                root = root.right


x = generate_tree()
print_tree(x)
print('先序遍历')
for f in (pre_order, Recursive.pre_order, Loop.pre_order):
    f(x)
    print()
print("\n中序遍历")
for f in (mid_order, Recursive.mid_order, Loop.mid_order):
    f(x)
    print()
print("\n后序遍历")
for f in (back_order, Recursive.back_order, Loop.back_order):
    f(x)
    print()
