package main

import (
	"fmt"
	"math"
	"regexp"
	"strconv"
	"strings"
)

/*
Get the index of the lowest float in the slice
*/
func argminNonZeroFloat(inSlice []float64) int {
	minIdx := 0
	minVal := inSlice[0]
	for ci, i := range inSlice {
		if i-0.0 > 1e-9 && i < minVal {
			minVal = i
			minIdx = ci
		}
	}
	return minIdx
}

/*
Calculate the mean of the data in the slice
*/
func mean(data []float64) float64 {
	total := 0.0
	for _, value := range data {
		total += value
	}
	return total / float64(len(data))
}

/*
calculate the distance in 2D space bewteen two points (sqrt not taken)
*/
func dist(ya float64, xa float64, yb float64, xb float64) float64 {
	return math.Pow((ya-yb), 2) + math.Pow((xa-xb), 2)

}

/*
get new grid positions after sec seconds for all input roboter
*/
func new_pos(lines []string, sec int, ydim int, xdim int) ([]float64, []float64) {
	// extract numbers from input
	r, _ := regexp.Compile("-?[0-9]{1,}")
	// new x and y positions
	var upd_x []float64
	var upd_y []float64
	for _, bot := range lines {
		if len(bot) == 0 {
			continue
		}
		pos := r.FindAllString(bot, -1)
		// x y velocity x velocity y
		var pos_int []int
		// convert input to int
		for _, p := range pos {
			p_int, err := strconv.Atoi(p)
			if err != nil {
				panic("cant convert to int")
			}
			pos_int = append(pos_int, p_int)
		}
		// offset of the current position after given seconds
		x_new := pos_int[0] + pos_int[2]*sec%xdim
		y_new := pos_int[1] + pos_int[3]*sec%ydim

		// wrap around the sides if not on grid
		if x_new < 0 {
			x_new = xdim + x_new
		} else if x_new >= xdim {
			x_new = x_new - xdim
		}

		if y_new < 0 {
			y_new = ydim + y_new
		} else if y_new >= ydim {
			y_new = y_new - ydim
		}
		upd_y = append(upd_y, float64(y_new))
		upd_x = append(upd_x, float64(x_new))

	}
	return upd_x, upd_y
}
func day14_p2(content string) {
	// calc center of gravity
	// check grid when all distances to cog are lower than usual
	lines := strings.Split(content, "\n")
	// y and x dimensions
	y := 103
	x := 101

	// check for the first 10000 seconds
	var mean_dists []float64
	for i := 0; i < 10000; i++ {
		// the new positions at second i
		ux, uy := new_pos(lines, i, y, x)
		// center of gravity
		cog_y := mean(uy)
		cog_x := mean(ux)
		var dists []float64
		// distances for all points to the cog
		for i := 0; i < len(ux); i++ {
			dists = append(dists, dist(float64(uy[i]), float64(ux[i]), cog_y, cog_x))
		}
		// mean dist to cog
		mean_dists = append(mean_dists, mean(dists))
	}
	// smallest dist to cog => more order => tree
	// index of smallest dist => second
	smallest_mdist := argminNonZeroFloat(mean_dists)
	fmt.Println(smallest_mdist)

}
func day14_p1(content string) {
	//	same as `new_pos`
	lines := strings.Split(content, "\n")
	y := 103
	x := 101
	sec := 100
	y_border := math.Floor(float64(y) / 2)
	x_border := math.Floor(float64(x) / 2)

	var quadrants [4]int
	r, _ := regexp.Compile("-?[0-9]{1,}")
	for _, bot := range lines {
		if len(bot) == 0 {
			continue
		}
		pos := r.FindAllString(bot, -1)
		// x y vx vy
		var pos_int []int
		for _, p := range pos {
			p_int, err := strconv.Atoi(p)
			if err != nil {
				panic("cant convert to int")
			}
			pos_int = append(pos_int, p_int)
		}
		x_new := pos_int[0] + pos_int[2]*sec%x
		y_new := pos_int[1] + pos_int[3]*sec%y

		if x_new < 0 {
			x_new = x + x_new
		} else if x_new >= x {
			x_new = x_new - x
		}

		if y_new < 0 {
			y_new = y + y_new
		} else if y_new >= y {
			y_new = y_new - y
		}
		// check in which quadrant the robot is
		if y_new < int(y_border) {
			if x_new < int(x_border) {
				quadrants[0] += 1
			} else if x_new > int(x_border) {
				quadrants[1] += 1
			}
		} else if y_new > int(y_border) {
			if x_new < int(x_border) {
				quadrants[2] += 1
			} else if x_new > int(x_border) {
				quadrants[3] += 1
			}

		}

	}
	sumq := 1
	for _, q := range quadrants {
		sumq *= q

	}
	fmt.Println(sumq)

}
