package solutions

import (
	"bufio"
	"fmt"
	"io"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"time"
)

// D4Solve ...
func D4Solve(input io.Reader) (p1 interface{}, p2 interface{}) {
	// setup
	lines := make([]string, 0)
	guards := map[int]*guard{}
	scanner := bufio.NewScanner(input)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	sort.Strings(lines)

	// pre process
	lastGuardID := 0
	for _, line := range lines {
		reTime := regexp.MustCompile(`\[(.*?)\] (.*)`)
		matchesTime := reTime.FindStringSubmatch(line)
		dt, _ := time.Parse("2006-01-02 15:04", matchesTime[1])
		reGuard := regexp.MustCompile(`Guard #(\d+) .*`)
		matchesGuard := reGuard.FindStringSubmatch(matchesTime[2])
		if len(matchesGuard) == 2 {
			lastGuardID, _ = strconv.Atoi(matchesGuard[1])
			_, ok := guards[lastGuardID]
			if !ok {
				guards[lastGuardID] = &guard{lastGuardID, make([]event, 1), map[string]int{}}
			}
			evt := event{dt.Hour(), dt.Minute(), eventBegin}
			guards[lastGuardID].events = append(guards[lastGuardID].events, evt)
		} else {
			var eventType int
			if strings.Contains(matchesTime[2], "falls") {
				eventType = eventSleep
			} else if strings.Contains(matchesTime[2], "wakes") {
				eventType = eventWake
			}
			evt := event{dt.Hour(), dt.Minute(), eventType}
			guards[lastGuardID].events = append(guards[lastGuardID].events, evt)
		}
	}

	// fill in guard sleep times
	for _, guard := range guards {
		for i, evt := range guard.events {
			if evt.eventType == eventSleep {
				currHour := evt.hour
				currMinute := evt.minute

				for {
					key := fmt.Sprintf("%d_%d", currHour, currMinute)

					_, ok := guard.minutes[key]
					if ok {
						guard.minutes[key]++
					} else {
						guard.minutes[key] = 1
					}

					currMinute++
					if currMinute > 59 {
						currMinute = 0
						currHour++
						if currHour > 23 {
							currHour = 0
						}
					}

					if currHour == guard.events[i+1].hour && currMinute == guard.events[i+1].minute {
						break
					}
				}
			}
		}
	}

	// strat 1
	sleepiestGuard := &guard{0, make([]event, 1), map[string]int{}}
	for _, guard := range guards {
		if totalSleepiness(guard) > totalSleepiness(sleepiestGuard) {
			sleepiestGuard = guard
		}
	}
	sleepiestMinute := ""
	for k, v := range sleepiestGuard.minutes {
		if sleepiestMinute == "" {
			sleepiestMinute = k
		}
		if v > sleepiestGuard.minutes[sleepiestMinute] {
			sleepiestMinute = k
		}
	}
	minutev, _ := strconv.Atoi(strings.Split(sleepiestMinute, "_")[1])
	p1 = sleepiestGuard.id * minutev

	// strat 2
	mostSleep := 0
	for _, guard := range guards {
		for k, min := range guard.minutes {
			if min > mostSleep {
				mostSleep = min
				sleepiestMinute = k
				sleepiestGuard = guard
			}
		}
	}
	minutev, _ = strconv.Atoi(strings.Split(sleepiestMinute, "_")[1])
	p2 = sleepiestGuard.id * minutev
	return
}

func totalSleepiness(g *guard) (sum int) {
	for _, min := range g.minutes {
		sum += min
	}
	return
}

type guard struct {
	id      int
	events  []event
	minutes map[string]int
}

type event struct {
	hour      int
	minute    int
	eventType int
}

const (
	eventBegin = iota
	eventSleep = iota
	eventWake  = iota
)
