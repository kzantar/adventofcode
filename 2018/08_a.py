

class Node:
    def __init__(self, child_cnt, meta_cnt, meta):
        self.child_cnt = child_cnt
        self.meta_cnt = meta_cnt
        self.meta = meta

    def __repr__(self):
        return f'<Node({self.child_cnt}, {self.meta_cnt}, {list(self.meta)})>'


items = [int(i) for i in open('08_input.txt').read().split()]


def build(items, childs):
    metas = []

    for _ in range(childs):
        child_cnt, meta_cnt = items[:2]
    
        if child_cnt > 0:
            items, new_metas = build(items[2:], child_cnt)
            metas.extend(new_metas)
            metas.extend(items[:meta_cnt])
            items = items[meta_cnt:]
        else:
            metas.extend(items[2:2+meta_cnt])
            items = items[2+meta_cnt:]

    return items, metas

    
_, metas = build(items, 1)
print(sum(metas))