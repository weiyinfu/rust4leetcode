import java.util.HashMap;
import java.util.Map;

class LRUCache {
class Node {
    int key, value;

    Node(int key, int value) {
        this.key = key;
        this.value = value;
    }

    Node next;
    Node prev;
}

class LinkedList {
    Node head = new Node(0, 0);
    Node tail = new Node(0, 0);

    LinkedList() {
        head.next = tail;
        tail.prev = head;
    }
}

LinkedList li = new LinkedList();
Map<Integer, Node> ma = new HashMap<>();
int cap = 0;
int len = 0;

public LRUCache(int capacity) {
    this.cap = capacity;
}

public int get(int key) {
    Node node = ma.get(key);
    if (node == null) return -1;
    remove(node);
    pushback(node);
    return node.value;
}

void pushback(Node node) {
    Node prev = li.tail.prev;
    prev.next = node;
    node.next = li.tail;
    li.tail.prev = node;
    node.prev = prev;
}

void remove(Node node) {
    Node prev = node.prev;
    Node next = node.next;
    prev.next = next;
    next.prev = prev;
}

public void put(int key, int value) {
    Node node = ma.get(key);
    if (node != null) {
        node.value = value;
        remove(node);
        pushback(node);
    } else {
        this.len++;
        if (this.len > this.cap) {
            this.len--;
            ma.remove(li.head.next.key);
            remove(li.head.next);
        }
        node = new Node(key, value);
        ma.put(key, node);
        pushback(node);
    }
}

public static void main(String[] args) {
    LRUCache cache = new LRUCache(2 /* capacity */);
    cache.put(1, 1);
    cache.put(2, 2);
    cache.get(1);       // returns 1
    cache.put(3, 3);    // evicts key 2
    cache.get(2);       // returns -1 (not found)
    cache.put(4, 4);    // evicts key 1
    cache.get(1);       // returns -1 (not found)
    cache.get(3);       // returns 3
    cache.get(4);       // returns 4
}
}
/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache obj = new LRUCache(capacity);
 * int param_1 = obj.get(key);
 * obj.put(key,value);
 */