import (
	"fmt"
)

func generateMatrix(n int) [][]int {
	ans := make([][]int, n)
	for i := 0; i < len(ans); i++ {
		ans[i] = make([]int, n)
	}
	x, y := 0, 0
	legal := func(x, y int) bool {
		return x >= 0 && y >= 0 && x < n && y < n
	}
	dir := [][]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	d := 0
	for i := 0; i < n*n; i++ {
		ans[x][y] = i + 1
		xx := dir[d][0] + x
		yy := dir[d][1] + y
		if !legal(xx, yy) || ans[xx][yy] != 0 {
			d = (d + 1) % 4
			xx = x + dir[d][0]
			yy = y + dir[d][1]
		}
		x, y = xx, yy
	}
	return ans
}