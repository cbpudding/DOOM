// Emacs style mode select   -*- C++ -*-
//-----------------------------------------------------------------------------
//
// $Id:$
//
// Copyright (C) 1993-1996 by id Software, Inc.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// DESCRIPTION:
//
//
//-----------------------------------------------------------------------------

#ifndef __F_FINALE__
#define __F_FINALE__

#include "d_event.h"
#include "doomtype.h"
//
// FINALE
//

// Called by main loop.
boolean F_Responder(event_t *ev);

// Called by main loop.
void F_Ticker(void);

// Called by main loop.
void F_Drawer(void);

void F_StartFinale(void);


#include "r_defs.h"
// Rust replaced data
extern int finalestage;
extern int finalecount;

extern char* finaletext;
extern char* finaleflat;

extern int castnum;
extern int casttics;
extern state_t *caststate;
extern boolean castdeath;
extern int castframes;
extern int castonmelee;
extern boolean castattacking;

// RS fns
void F_StartFinale(void);
boolean F_Responder(event_t *event);
void F_Ticker(void);
void F_TextWrite(void);
void F_StartCast(void);
void F_CastTicker(void);
boolean F_CastResponder(event_t *ev);
void F_CastPrint(char *text);
void F_CastDrawer(void);
void F_DrawPatchCol(int x, patch_t *patch, int col);
void F_BunnyScroll(void);
void F_Drawer(void);

// Orig fns
void OF_StartFinale(void);
boolean OF_Responder(event_t *event);
void OF_Ticker(void);
void OF_TextWrite(void);
void OF_StartCast(void);
void OF_CastTicker(void);
boolean OF_CastResponder(event_t *ev);
void OF_CastPrint(char *text);
void OF_CastDrawer(void);
void OF_DrawPatchCol(int x, patch_t *patch, int col);
void OF_BunnyScroll(void);
void OF_Drawer(void);


#endif
//-----------------------------------------------------------------------------
//
// $Log:$
//
//-----------------------------------------------------------------------------
