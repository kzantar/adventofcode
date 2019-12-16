

class Node:
    def __init__(self):
        self.child_cnt = 0
        self.meta_cnt = 0
        self.meta = []
        self.children = []

    def __repr__(self):
        return f'<Node({self.child_cnt}, {self.meta_cnt}, {list(self.meta)})>'

    def __str__(self):
        return self.__repr__()

    def add_child(self, node):
        self.children.append(node)

    def add_meta(self, meta):
        self.meta = meta

    @property
    def has_children(self):
        return self.child_cnt > 0

    def build(self, items):
        self.child_cnt, self.meta_cnt = items[:2]
        items = items[2:]

        for _ in range(self.child_cnt):
            node = Node()
            items = node.build(items)
            self.add_child(node)

        meta = items[:self.meta_cnt]
        self.add_meta(meta)
        items = items[self.meta_cnt:]

        return items

    def draw(self, ident=' '):
        print(ident, self)
        for child in self.children:
            child.draw(ident=ident+'  ')

    def sum_meta(self):
        metas = self.meta
        for child in self.children:
            metas.extend(child.sum_meta())

        return metas

    @property
    def value(self):
        if not self.has_children:
            return sum(self.meta)

        values = []
        for i in self.meta:
            if i == 0:
                continue

            try:
                child = self.children[i-1]
                values.append(child.value)
            except IndexError:
                pass

        return sum(values)


if __name__ == '__main__':
    items = [int(i) for i in open('08_input.txt').read().split()]

    root = Node()
    result = root.build(items)
    print(root.value)