#include <iostream>
#include <memory> // Pour std::unique_ptr et std::nullptr

struct Node {
  int val;
  std::unique_ptr<Node> next;

  Node(int v, std::unique_ptr<Node> n) : val(v), next(std::move(n)) {}
};

int main() {
  std::unique_ptr<Node> head = nullptr;

  head = std::make_unique<Node>(100, std::move(head));
  head = std::make_unique<Node>(200, std::move(head));

  const Node *current = head.get(); // get() pour avoir un pointeur brut sans
                                    // transfert de possession

  while (current != nullptr) {
    std::cout << current->val << std::endl;
    current = current->next.get();
  }

  return 0;
}