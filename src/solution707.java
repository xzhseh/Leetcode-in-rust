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
    private class Node {    
        private int val;
        private Node next;
        private Node prev;

        public void Node(int val, Node next, Node prev) {
            val = val;
            next = next;
            prev = prev;
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
        if index >= size {
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

    }
    
    public void addAtTail(int val) {

    }
    
    public void addAtIndex(int index, int val) {

    }
    
    public void deleteAtIndex(int index) {

    }
}