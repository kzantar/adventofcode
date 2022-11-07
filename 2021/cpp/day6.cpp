#include <iostream>
#include <fstream>
#include <stdexcept>
#include <regex>
#include <map>

using namespace std;

typedef map<int, uint64_t> Swarm;


void print(const Swarm& s) {
	for (const auto& [age, cnt] : s) {
		cout << age << " -> " << cnt << "; ";
	}
	cout << endl;
}

uint64_t sum(const Swarm& swarm) {
	uint64_t s = 0;
	for (const auto& [age, cnt] : swarm) {
		s += cnt;
	}

	return s;
}


Swarm update(const Swarm& s) {
	Swarm swarm;
	for (auto& [age, cnt] : s) {
		if (age > 0 && age <= 8) {
			swarm[age-1] += cnt;
		}
		else if (age == 0) {
			swarm[6] += cnt;
			swarm[8] = cnt;
		}
	}

	return swarm;
}


uint64_t answer1(Swarm swarm) {
	cout << "Initial state: ";
	print(swarm);
	for (int day = 1; day <= 80; day++) {
		swarm = update(swarm);
//		cout << "After " << setw(3) << day << ": ";
//		print(swarm);
	}

	return sum(swarm);
}

uint64_t answer2(Swarm swarm) {
	cout << "Initial state: ";
	print(swarm);
	for (int day = 1; day <= 256; day++) {
		swarm = update(swarm);
//		cout << "After " << setw(3) << day << ": ";
//		print(swarm);
	}
	print(swarm);

	return sum(swarm);
}

int main() {
	string filename = "../puzzle/6_input.txt";
	ifstream ifs(filename);
	if (!ifs.is_open()) {
		cerr << "failed to open " << filename << endl;
		return 1;
	}

	Swarm swarm;
	string token;
	while (getline(ifs, token, ',')) {
		int num = stoi(token, nullptr);
		swarm[num]++;
	}

	cout << "Answer 1: \n" << answer1(swarm) << endl;
	cout << "Answer 2: \n" << answer2(swarm) << endl;
}
