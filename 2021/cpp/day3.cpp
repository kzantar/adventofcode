#include <iostream>
#include <fstream>
#include <vector>
#include <stdexcept>
#include <bitset>

using namespace std;

constexpr int L = 12;

typedef bitset<L> Value;
typedef vector<Value> Values;


u_long answer1(const Values& values) {
	Value gamma;
	for (int j=0; j < L; ++j) {
		int ones = 0;
		for (int i=0; i < values.size(); ++i) {
			if (values[i][j]) ++ones;
		}
		gamma[j] = ones > values.size()/2;
	}
	Value epsilon = ~gamma;

	return gamma.to_ulong() * epsilon.to_ulong();
}

int most_common_bit(const Values& values, int num_bit) {
	int ones = 0;
	int zeros = 0;
	for (auto v : values) {
		v[num_bit] ? ++ones : ++zeros;
	}

	return (ones >= zeros) ? 1 : 0;
}

int least_common_bit(const Values& values, int num_bit) {
	int ones = 0;
	int zeros = 0;
	for (auto v : values) {
		v[num_bit] ? ++ones : ++zeros;
	}

	return (zeros <= ones) ? 0 : 1;
}

u_long answer2(const Values& values) {
	Values oxy_values = values;
	int num_bit = L - 1;
	while (oxy_values.size() != 1 && num_bit >= 0) {
		int common = most_common_bit(oxy_values, num_bit);
		erase_if(oxy_values, [&](Value v) {
			return v[num_bit] != common;
		});
		num_bit--;
	}
	Value oxygen = oxy_values[0];
	cout << "oxygen: " << oxygen << endl;

	Values co2_values = values;
	num_bit = L - 1;
	while (co2_values.size() != 1 && num_bit >= 0) {
		int common = least_common_bit(co2_values, num_bit);
		erase_if(co2_values, [&](Value v){
			return v[num_bit] != common;
		});
		num_bit--;
	}
	Value co2 = co2_values[0];
	cout << "co2: " << co2 << endl;

	return oxygen.to_ulong() * co2.to_ulong();
}

int main() {
	string filename = "../3_input.txt";
	ifstream ifs(filename);
	if (!ifs.is_open()) {
		cerr << "failed to open " << filename << endl;
		return 1;
	}

	Values values;
	Value rec;
	while (ifs >> rec) {
		values.push_back(rec);
	}

	cout << "Answer 1: " << answer1(values) << endl;
	cout << "Answer 2: " << answer2(values) << endl;

}