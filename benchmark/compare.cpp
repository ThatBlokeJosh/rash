#include <iostream>

int main() {
	for (int i = 0; i < 100; i++) {
		for (int ii = 0; ii < 100; ii++) {
			for (int iii = 0; iii < 100; iii++) {
				std::cout << i*ii*iii << std::endl;
			}
		}
	}
}
