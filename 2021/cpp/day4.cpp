#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <stdexcept>
#include <algorithm>

using namespace std;

constexpr int N = 5;

const string red("\033[0;31m");
const string reset("\033[0m");

class Board {
public:
	Board() {
		table = vector<vector<int>>(N, vector<int>(N));
		mask = vector<vector<bool>>(N, vector<bool>(N, false));
	};

	void set(int num) {
		for (int i=0; i<N; ++i) {
			for (int j=0; j<N; ++j) {
				if (table[i][j] == num) {
					mask[i][j] = true;
				}
			}
		}
	}

	bool is_win() {
		for (auto& row : mask) {
			if (all_of(row.begin(), row.end(), [](bool v){ return v; })) {
				return true;
			}
		}

		for (int j=0; j<N; ++j) {
			bool col = true;
			for (int i=0; i<N; ++i) {
				col &= mask[i][j];
			}
			if (col) return true;
		}

		return false;
	}

	int sum_unmarked() {
		int sum = 0;
		for (int i=0; i<N; ++i) {
			for (int j=0; j<N; ++j) {
				if (!mask[i][j]) sum += table[i][j];
			}
		}
		return sum;
	}

	friend istream& operator>>(istream& input, Board& b) {
		for (int i = 0; i < N; ++i) {
			for (int j = 0; j < N; ++j) {
				input >> b.table[i][j];
			}
		}
		return input;
	}

	friend ostream& operator<<(ostream& out, const Board& b) {
		for (int i = 0; i < N; ++i) {
			for (int j = 0; j < N; ++j) {
				if (b.mask[i][j]) {
					out << red << b.table[i][j] << reset << " ";
				} else {
					out << b.table[i][j] << " ";
				}
			}
			out << endl;
		}
		return out;
	}

private:
	vector<vector<int>> table;
	vector<vector<bool>> mask;
};


int answer1(const vector<int>& nums, vector<Board> boards) {
	int score;
	for (int n : nums) {
		for (auto& b : boards) {
			b.set(n);
			if (b.is_win()) {
				score = b.sum_unmarked() * n;
				goto endloop;
			}
		}
	}
	endloop:
		return score;
}


int answer2(const vector<int>& nums, vector<Board> boards) {
	int score;
	for (int n : nums) {
		for (auto& b : boards) {
			b.set(n);
			if (all_of(boards.begin(), boards.end(), [](Board b){
				return b.is_win();
			})) {
				cout << b << endl;
				score = b.sum_unmarked() * n;
				goto endloop;
			}
		}
	}
	endloop:
	return score;
}


int main() {
	string filename = "../4_input.txt";
	ifstream ifs(filename);
	if (!ifs.is_open()) {
		cerr << "failed to open " << filename << endl;
		return 1;
	}

	string first_line;
	ifs >> first_line;

	stringstream ss(first_line);
	string token;
	vector<int> nums;
	while (getline(ss, token, ',')) {
		nums.push_back(stoi(token, nullptr));
	}

	vector<Board> boards;
	while (ifs.good()) {
		Board b;
		ifs >> b;
		boards.push_back(b);
	}

	cout << "Answer 1: " << answer1(nums, boards) << endl;
	cout << "Answer 2: " << answer2(nums, boards) << endl;


//	for (auto& b : boards) {
//		b.set(24);
//		cout << b;
//		cout << endl;
//	}

}
