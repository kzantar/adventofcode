#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <sstream>
#include <map>
#include <array>
#include <bitset>

using namespace std;

typedef string Set;
typedef vector<Set> Signals;
typedef vector<Set> Outputs;
typedef bitset<7> Segments;

const string DIGIT = {
	" 0000 \n"
	"1    2\n"
	"1    2\n"
	" 3333 \n"
	"4    5\n"
	"4    5\n"
	" 6666 \n"
};

map<int, Segments> DIGITS = {
	{0, 0b1110111},
	{1, 0b0010010},
	{2, 0b1011101},
	{3, 0b1011011},
	{4, 0b0111010},
	{5, 0b1101011},
	{6, 0b1101111},
	{7, 0b1010010},
	{8, 0b1111111},
	{9, 0b1111011},
};


bool is_digit(const string &digit) {
	switch (digit.size()) {
		case 2: case 3: case 4: case 7:
			return true;
		default:
			return false;
	}
}

Set operator-(const Set &s1, const Set &s2) {
	Set result;
	for (auto v : s1) {
		if (!s2.contains(v)) result += v;
	}
	return result;
}


struct Suite {
	Signals signals;
	Outputs outputs;
	map<string, Segments> sig2seg;
	map<u_long, string> seg2sig;

	void solve() {
		for (auto &sig : signals) {
			switch (sig.size()) {
				case 2:
					seg2sig[DIGITS[1].to_ulong()] = sig;
					sig2seg[sig] = DIGITS[1];
					break;
				case 4:
					seg2sig[DIGITS[4].to_ulong()] = sig;
					sig2seg[sig] = DIGITS[4];
					break;
				case 3:
					seg2sig[DIGITS[7].to_ulong()] = sig;
					sig2seg[sig] = DIGITS[7];
					break;
				case 7:
					seg2sig[DIGITS[8].to_ulong()] = sig;
					sig2seg[sig] = DIGITS[8];
					break;
			}
		}
	}

	friend istream& operator>>(istream& input, Suite& suite) {
		string line;
		getline(input, line);
		stringstream ss{line};
		string signal;
		for (int i = 0; i < 10; ++i) {
			ss >> signal;
			suite.signals.push_back(signal);
		}
		string output;
		ss >> output;
		for (int i = 0; i < 4; ++i) {
			ss >> output;
			suite.outputs.push_back(output);
		}

		return input;
	}

	friend ostream& operator<<(ostream &out, const Suite& suite) {
		out << "Signals to segments: \n";
		for (const auto& [sig, seg] : suite.sig2seg) {
			out << sig << ": " << seg << "; ";
		}
		out << endl;

		out << "Segments to signals: \n";
		for (const auto& [seg, sig] : suite.seg2sig) {
			Segments s{seg};
			out << s << ": " << sig << "; ";
		}
		out << endl;

		return out;
	}
};


int answer1(const vector<string> &digits) {

	int count = 0;
	for (auto &d : digits) {
		if (is_digit(d)) count++;
	}
	return count;
}


int answer2() {
	return 0;
}


int main() {
	string filename = "../puzzle/8_test.txt";
	ifstream ifs(filename);
	if (!ifs.is_open()) {
		cerr << "failed to open " << filename << endl;
		return 1;
	}

	Suite suite;
	ifs >> suite;

	suite.solve();

	cout << suite << endl;

	cout << (DIGITS[7] ^ DIGITS[1]).to_string();


//	cout << "Signals: \n";
//	for (auto &s : signals) {
//		cout << s << endl;
//	}
//	cout << "\nDigits: \n";
//	for (auto &d : digits) {
//		cout << d << endl;
//	}
//	cout << endl;

//	cout << "Answer 1: \n" << answer1(digits) << endl;
//	cout << "Answer 2: \n" << answer2() << endl;
}