
#include <iostream>

struct Bird {
    Bird() {
        std::cout << "Bird()\n";
    }

    ~Bird() noexcept {
        std::cout << "~Bird()\n";
    }

    void fly() const {
        std::cout << "fly()\n";
    }
};

static auto bird = Bird{};

int main() {
    bird.fly();
}
