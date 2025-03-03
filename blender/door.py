# enum DoorOpenType {
# 	SWING_BACK,
# 	SWING_FORWARD,
# 	SLIDE_FORWARD_1,
# 	SLIDE_FORWARD_2,
# 	SLIDE_FORWARD_3,
# 	SLIDE_FORWARD_4,
# 	SWING_BACK_RETURN_1,
# 	SWING_BACK_RETURN_2,
# 	SWING_BACK_RETURN_3,
# 	SWING_BACK_RETURN_4,
# 	SLIDE_RETURN,
# 	ELEVATOR,
# 	INVISIBLE,
# 	SAW_CONTINUOUS,
# 	SAW_PAUSE,
# 	SPEAR_VERTICAL,
# 	SPEAR_HORIZONTAL,
# 	PENDULUM,
# 	BLADE,
# 	CRUSH_VERTICAL,
# 	CRUSH_VERTICAL_FAST,
# 	CRUSh_VERTICAL_SLOW,
# 	SLIDE_CRUSH_1,
# 	SLIDE_CRUSH_2,
# 	SLIDE_CRUSH_3,
# 	STATIC
# }

# func get_open_type() -> int:
# 	match(opentype):
# #0 	normal 90 degree door swing backward 	
# #1 	normal 90 degree door swing backward 	
# #2 	normal 90 degree door swing backward 	
# #3 	normal 90 degree door swing backward 	
# #4 	normal 90 degree door swing backward 	
# #5 	normal 90 degree door swing forward 	
# #6 	normal 90 degree door swing forward 	
# #7 	normal 90 degree door swing forward 	
# #8 	normal 90 degree door swing backward 	
# 		0,1,2,3,4,5,9:
# 			return DoorOpenType.SWING_BACK
# 		8:
# 			return DoorOpenType.SWING_FORWARD
# #10 	slides forward 	
# #11 	slides forward 	
# #12 	slides forward 	
# 		10,11,12:
# 			return DoorOpenType.SLIDE_FORWARD_1
# #15 	slides further forward 	
# #16 	slides further forward 	
# #17 	slides further forward 	
# 		15,16,17:
# 			return DoorOpenType.SLIDE_FORWARD_2
# #20 	slides even further forward 	
# #21 	slides even further forward 	
# #22 	slides even further forward 	
# 		20,21,22:
# 			return DoorOpenType.SLIDE_FORWARD_3
# #25 	slides furthest forward 	
# #26 	slides furthest forward 	
# #27 	slides furthest forward 	
# 		25,26,27:
# 			return DoorOpenType.SLIDE_FORWARD_4
# #30 	rotates 90 degrees clockwise and returns 	
# 		30:
# 			return DoorOpenType.SWING_BACK_RETURN_1
# #35 	rotates 90 degrees clockwise and returns faster 	
# #36 	rotates 90 degrees and jumps back 	
# 		35:
# 			return DoorOpenType.SWING_BACK_RETURN_2
# 		36:
# 			return DoorOpenType.SWING_BACK_RETURN_3
# #40 	rotates 90 degrees clockwise and returns slower 	
# 		40:
# 			return DoorOpenType.SWING_BACK_RETURN_4
# #45 	slide sideways open and closes slowly 	
# 		45:
# 			return DoorOpenType.SLIDE_RETURN
# 		50,53,54:
# 			return DoorOpenType.INVISIBLE
# 		115:
# 			return DoorOpenType.SAW_CONTINUOUS
# 		116:
# 			return DoorOpenType.SAW_PAUSE
# 		120:
# 			return DoorOpenType.SPEAR_VERTICAL
# 		125:
# 			return DoorOpenType.SPEAR_HORIZONTAL
# 		130:
# 			return DoorOpenType.PENDULUM
# 		135:
# 			return DoorOpenType.BLADE
# 		140:
# 			return DoorOpenType.CRUSH_VERTICAL
# 		142,143,144:
# 			return DoorOpenType.ELEVATOR
# 		145:
# 			return DoorOpenType.CRUSH_VERTICAL_SLOW
# 		146:
# 			return DoorOpenType.CRUSH_VERTICAL_FAST
# 		150:
# 			return DoorOpenType.SLIDE_CRUSH_1
# 		151:
# 			return DoorOpenType.SLIDE_CRUSH_2
# 		152:
# 			return DoorOpenType.SLIDE_CRUSH_3
# 		_:
# 			return DoorOpenType.STATIC
