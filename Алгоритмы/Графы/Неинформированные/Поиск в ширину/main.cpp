#include <queue>

struct Node {
    int value;
    Node* left;
    Node *right;

    bool visited{false};
};

Node* bfs(Node* root, int destination) {
    root->visited = true;

    std::queue<Node*> queue;
    queue.emplace(root);

    while(!queue.empty()) {
        Node* node = queue.front(); queue.pop();
        if(node->value == destination) {
            return node;
        }

        if(node->left != nullptr && !node->visited) {
            queue.push(node->left);
            node->left->visited = true;
        }
        if(node->right != nullptr && !node->visited) {
            queue.push(node->right);
            node->right->visited = true;
        }
    }

    return nullptr;
}
