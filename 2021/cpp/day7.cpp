#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <numeric>

using namespace std;

typedef vector<int> Positions;

int median(Positions v) {
	size_t n = v.size() / 2;
	nth_element(v.begin(), v.begin()+n, v.end());
	return v[n];
}

int average(Positions &v) {
	double avg = accumulate(v.begin(), v.end(), 0.0) / v.size();

	return static_cast<int>(floor(avg));
}


int total_fuel(const Positions &positions, int pos, int (*cost)(int, int)) {
	int total = 0;
	for (auto &p : positions) {
		total += cost(p, pos);
	}

	return total;
}


int cost_1(const int start, const int end) {
	return abs(end - start);
}


int cost_2(const int start, const int end) {
	int dx = abs(end - start);
	int total = 0;
	for (int i = 0; i <= dx; ++i) {
		total += i;
	}

	return total;
}

int answer1(Positions &positions) {
	int pos = median(positions);
	return total_fuel(positions, pos, cost_1);
};

int answer2(Positions &positions) {
	cout << "average: " << average(positions) << endl
		 << "median: " << median(positions) << endl;

	int pos = average(positions);
	return total_fuel(positions, pos, cost_2);
};

int answer2_2(Positions &positions) {
	const auto [min_pos, max_pos] = minmax_element(positions.begin(), positions.end());
	cout << "min: " << *min_pos << " max: " << *max_pos << endl;

	int min_fuel = INT_MAX;
	for (int i = *min_pos; i <= *max_pos; ++i) {
		int cur_fuel = total_fuel(positions, i, cost_2);
		if (cur_fuel < min_fuel) {
			min_fuel = cur_fuel;
		}
	}
	return min_fuel;
};


int main() {
	string filename = "../puzzle/7_input.txt";
	ifstream ifs(filename);
	if (!ifs.is_open()) {
		cerr << "failed to open " << filename << endl;
		return 1;
	}

	Positions positions;
	string token;
	while (getline(ifs, token, ',')) {
		int pos = stoi(token, nullptr);
		positions.push_back(pos);
	}

	cout << "Answer 1: \n" << answer1(positions) << endl;
	cout << "Answer 2: \n" << answer2(positions) << endl;
	cout << "Answer 2_2: \n" << answer2_2(positions) << endl;
}