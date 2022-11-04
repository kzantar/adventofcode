#include <iostream>
#include <fstream>
#include <vector>
#include <stdexcept>
#include <regex>


using namespace std;


int sign(int a) {
	if (a == 0) return 0;
	if (a > 0)
		return 1;
	else
		return -1;
}


int answer1();


int answer2();


struct Point {
	int x;
	int y;
	int ch;

	Point(): x(0), y(0), ch(0) {}
	Point(int a, int b): x(a), y(b), ch(0) {}
	Point(int a, int b, int c): x(a), y(b), ch(c) {}
};


struct Line {
	Point start;
	Point end;

	Line(): start(Point(0, 0)), end(Point(0, 0)) {}

	Line(Point s, Point e): start(s), end(e) {}

	Line(int x1, int y1, int x2, int y2)
	: start(Point(x1, y1))
	, end(Point(x2, y2))
	{}

	bool is_straight() {
		return (start.x == end.x) || (start.y == end.y);
	}

	friend istream& operator>>(istream& input, Line& l) {
		string s;
		getline(input, s);

		if (s.empty()) return input;

		regex pattern(R"((\d+),(\d+)\s+->\s+(\d+),(\d+))");
		smatch m;
		regex_match(s, m, pattern);

		if (m.ready()) {
			l.start.x = stoi(m.str(1));
			l.start.y = stoi(m.str(2));
			l.end.x = stoi(m.str(3));
			l.end.y = stoi(m.str(4));
		}

		return input;
	}

	friend ostream& operator<<(ostream& out, const Line& l) {
		out << l.start.x << ","
			<< l.start.y << " -> "
			<< l.end.x << ","
			<< l.end.y << endl;

		return out;
	}

};


struct Board {
	vector<Line> lines;
	vector<vector<Point>> points;

	Board() = default;

	void add_line(Line line) {
		lines.push_back(line);
	}

	void draw() {
		for (auto& row : points) {
			for (auto& p : row) {
				cout << p.ch;
			}
			cout << endl;
		}
	}

	void load(ifstream& ifs) {
		Line line;
		while (ifs >> line) {
			this->add_line(line);
		}
	}

	void set_size(size_t size) {
		points.clear();
		for (int i = 0; i < size; ++i) {
			vector<Point> row;
			for (int j = 0; j < size; ++j) {
				row.emplace_back(i, j, 0);
			}
			points.push_back(row);
		}
	}

	void draw_line(const Line& line) {
		int start_x = line.start.x;
		int start_y = line.start.y;
		int end_x = line.end.x;
		int end_y = line.end.y;

		points[end_y][end_x].ch++;
		while( start_x != end_x || start_y != end_y) {
			points[start_y][start_x].ch++;
			start_x += sign(end_x - start_x);
			start_y += sign(end_y - start_y);
		}
	}

	void update() {
		for (auto& line : lines) {
//			if (!line.is_straight()) continue;
			draw_line(line);
		}
	}

	int count_overlap() {
		int cnt = 0;
		for (auto& row : points)
			for (auto& p : row)
				if (p.ch > 1) cnt++;

		return cnt;
	}
};


int main() {
	string filename = "../puzzle/5_input.txt";
	ifstream ifs(filename);
	if (!ifs.is_open()) {
		cerr << "failed to open " << filename << endl;
		return 1;
	}

	Board board;
	board.load(ifs);

	board.set_size(1000);
	board.update();

//	board.draw();

	cout << "Answer: " << board.count_overlap() << endl;
}