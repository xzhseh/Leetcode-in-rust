class Solution {
    private int index = 0;

    public String decodeString(String s) {
        LinkedList<String> stack = new LinkedList<String>();

        while (index < s.length()) {
            char cur_char = s.charAt(index);

            if (Character.isDigit(cur_char)) {
                String digits = getDigits(s);
                stack.addLast(digits);
            } else if (Character.isLetter(cur_char) || cur_char == '[') {
                stack.addLast(String.valueOf(s.charAt(index++)));
            } else { // The case for ']'
                index += 1;
                LinkedList<String> tmp_list = new LinkedList<String>();

                while (!"[".equals(stack.peekLast())) {
                    tmp_list.addLast(stack.removeLast());
                }

                Collections.reverse(tmp_list);
                stack.removeLast(); // Remove '['
                
                int the_num = Integer.parseInt(stack.removeLast());
                StringBuffer tmp_buffer = new StringBuffer();
                String add_string = getString(tmp_list);

                for (int i = 0; i < the_num; ++i) {
                    tmp_buffer.append(add_string);
                }
                
                stack.addLast(tmp_buffer.toString());
            }  
        }

        return getString(stack);
    }

    private String getDigits(String s) {
        StringBuffer ret = new StringBuffer();

        while (Character.isDigit(s.charAt(index))) {
            ret.append(s.charAt(index++));
        }

        return ret.toString();
    }

    private String getString(LinkedList<String> v) {
        StringBuffer ret = new StringBuffer();
        for (String s : v) {
            ret.append(s);
        }
        return ret.toString();
    }
}