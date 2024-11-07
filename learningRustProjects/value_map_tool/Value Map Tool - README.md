# Value Map Tool

## Project Proposal
[[2024-08-19]] @ 17:03 
As my first rust project I want to make a command line tool to do various common value map transformations.
This is inspired by the various python functions I have written over the past month to prepare value maps for Deimos.

The first step will be transforming a list with a date, time and value column and `=365*48` values like below:

```csv
date, time, value 
YYYY/MM/dd, HH:mm, float
YYYY/MM/dd, HH:mm+30, float
...
```

into a value map form with a date column followed by 48 value columns with 365 daily rows like below:

```csv
date,00:00,00:30,01:00,01:30,02:00,02:30,03:00,03:30,04:00,04:30,05:00,05:30,06:00,06:30,07:00,07:30,08:00,08:30,09:00,09:30,10:00,10:30,11:00,11:30,12:00,12:30,13:00,13:30,14:00,14:30,15:00,15:30,16:00,16:30,17:00,17:30,18:00,18:30,19:00,19:30,20:00,20:30,21:00,21:30,22:00,22:30,23:00,23:30:00
YYYY/MM.dd,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float,float 
...
```

## Development Targets
### Bugs
```tasks
path includes {{query.file.path}}
tags include #bug-report 
short mode
hide tags
```
### Features 
```tasks
path includes {{query.file.path}}
tags include #feature-request
short mode
hide tags
```

# Development Log

## Day 1 Hour 0
[[2024-08-19]] @ 17:32 
The #goal for today is to:
- [ ] Convert list to Value Map #feature-request 

this is dependant on the ability to:
- [ ] Load a CSV #feature-request 
For this I will need the [CSV Crate](https://docs.rs/csv/latest/csv/)


Further features will be to:
- [ ] Be able to deal with different date formats #feature-request
	- "dd/MM/YYYY"
	- "MM/dd/YYYY"
	- "-" instead of "/" date deliminator

- [ ] Be able to deal with arbitrary time indexes like `[0,47]` #feature-request 

