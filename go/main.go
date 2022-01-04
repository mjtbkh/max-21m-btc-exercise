package main

import "fmt"

const (
	HalvingPeriod uint32 = 210000
	InitialRewardSize uint64 = 50*100000000
)

func main()  {
	var CurrentRewardSize uint64 = InitialRewardSize
	var TotalRewards uint64 = 0
	var BlockNumber uint32 = 0
	var HalvingNumber uint16 = 0

	for {
		CurrentRewardSize = InitialRewardSize >> HalvingNumber
		TotalRewards += CurrentRewardSize
		BlockNumber++

		fmt.Printf("block\t#%d, rewarded\t%d\n", BlockNumber, CurrentRewardSize)

		if BlockNumber % HalvingPeriod == 0 {
			// every 210000 blocks, halving happens
			HalvingNumber++
			fmt.Println("halving occurred!")
		}

		if CurrentRewardSize == 0 {
			// at some point block reward size will reach 0, we want to break the loop at that point
			break
		}
	}

	fmt.Printf("sum of mined rewards until block #%d: %d\n", BlockNumber, TotalRewards)
	fmt.Printf("total halvings: %d\n", HalvingNumber)
}
