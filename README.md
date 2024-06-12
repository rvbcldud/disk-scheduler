# Disk Scheduling Simulator

_This program was written for my undergraduate operating systems class_

Simulates the execution of several disk scheduling algorithms:
- First Come First Server (FCFS)
- Shortest Seek Time First (SSTF)
- SCAN (Elevator Algorithm)
  - With its circular counterpart, C-SCAN
- LOOK
  - With its circular counterpart, C-LOOK

## Compile

In order to compile the Rust package with cargo with ease, I have provided a makefile, which is invoked with the following:
```console
$ make
```
This compiles the Rust package and copies the executable binary `DiskSim` to the directory it is called from.

## Usage

There are a few ways in which the simulation can be used:

### Random
To generate a random head position (0 -> 10000 exclusive), an initial direction, and 100 random cylinder requests:
```console
$ ./DiskSim R
```
### Semi-random
To generate a series of `n` random disk requests (0 -> 10000 exclusive):
```console
$ ./DiskSim 20 H Rn
```
This would run a simulation that starts the disk head at location cylinder 20 initially going from low to high, with `n` randomly generated cylinder requests.
### Manual
To start the simulation with manual inputs:
```console
$ ./DiskSim 230 L 2500 20 50
```
Where 230 is the initial location of the disk head, L denotes the location going from high to low, followed by a series of cylinder requests (in this case, 2500, 20, 50)

### Example Output
```console
$ ./DiskSim 200 H R5
== Service History ==
FCFS 4905 4358 1352 1431 3290 
SSTF 1352 1431 3290 4358 4905 
SCAN 1352 1431 3290 4358 4905 
C-SCAN 1352 1431 3290 4358 4905 
LOOK 1352 1431 3290 4358 4905 
C-LOOK 1352 1431 3290 4358 4905 
== Service Stats ==
FCFS 2 10197
SSTF 0 4706
SCAN 0 4706
C-SCAN 0 4706
LOOK 0 4706
C-LOOK 0 4706
```

```console
$ ./DiskSim 200 H 400 900 100
== Service History ==
FCFS 400 900 100 
SSTF 100 400 900 
SCAN 400 900 100 
C-SCAN 400 900 100 
LOOK 400 900 100 
C-LOOK 400 900 100 
== Service Stats ==
FCFS 1 1501
SSTF 2 901
SCAN 1 19699
C-SCAN 2 19899
LOOK 1 1501
C-LOOK 2 1501
```
