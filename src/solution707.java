/**
 * Your MyLinkedList object will be instantiated and called as such:
 * MyLinkedList obj = new MyLinkedList();
 * int param_1 = obj.get(index);
 * obj.addAtHead(val);
 * obj.addAtTail(val);
 * obj.addAtIndex(index,val);
 * obj.deleteAtIndex(index);
 */
class MyLinkedList {
    private static class Node {    
        private int val;
        private Node next;
        private Node prev;

        Node(int _val, Node _next, Node _prev) {
            val = _val;
            next = _next;
            prev = _prev;
        }
    }

    private Node firstNode;
    private Node lastNode;
    private int size;

    public MyLinkedList() {
        firstNode = new Node(0, null, null);
        lastNode = new Node(0, null, firstNode);
        firstNode.next = lastNode;
        size = 0;
    }
    
    public int get(int index) {
        if (index >= size) {
            return -1;
        } else {
            Node tmpNode = firstNode.next;
            for (int i = 0; i < index; ++i) {
                tmpNode = tmpNode.next;
            }
            return tmpNode.val;
        }
    }
    
    public void addAtHead(int val) {
        Node newNode = new Node(val, firstNode.next, firstNode);
        firstNode.next = newNode;
        newNode.next.prev = newNode;
        size += 1;
    }
    
    public void addAtTail(int val) {
        Node newNode = new Node(val, lastNode, lastNode.prev);
        lastNode.prev = newNode;
        newNode.prev.next = newNode;
        size += 1;
    }
    
    public void addAtIndex(int index, int val) {
        if (index <= 0) {
            addAtHead(val);
        } else if (index == size) {
            addAtTail(val);
        } else if (index > size) {
            return;
        } else {
            Node tmpNode = firstNode.next;
            for (int i = 0; i < index - 1; ++i) {
                tmpNode = tmpNode.next;
            }
            Node newNode = new Node(val, tmpNode.next, tmpNode);
            tmpNode.next = newNode;
            newNode.next.prev = newNode;
            size += 1;
        }
    }
    
    public void deleteAtIndex(int index) {
        if (index < 0 || index >= size) {
            return;
        } else {
            Node tmpNode = firstNode;
            for (int i = 0; i < index; ++i) {
                tmpNode = tmpNode.next;
            }
            tmpNode.next = tmpNode.next.next;
            tmpNode.next.prev = tmpNode;
            size -= 1;
        }
    }
}