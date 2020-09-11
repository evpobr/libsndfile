/*
** Copyright (C) 2009-2017 Erik de Castro Lopo <erikd@mega-nerd.com>
**
** This program is free software; you can redistribute it and/or modify
** it under the terms of the GNU Lesser General Public License as published by
** the Free Software Foundation; either version 2.1 of the License, or
** (at your option) any later version.
**
** This program is distributed in the hope that it will be useful,
** but WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
** GNU Lesser General Public License for more details.
**
** You should have received a copy of the GNU Lesser General Public License
** along with this program; if not, write to the Free Software
** Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
*/

/*
**	This needs to be a separate file so that we don't have to include
**	<windows.h> elsewhere (too many symbol clashes).
*/


#include "sfconfig.h"

#if OS_IS_WIN32
#include <windows.h>

#define ENABLE_SNDFILE_WINDOWS_PROTOTYPES 1
#include "sndfile.h"
#include "common.h"

extern int sf_errno ;

SNDFILE*
sf_wchar_open (LPCWSTR wpath, int mode, SF_INFO *sfinfo)
{	SF_PRIVATE 	*psf ;

	if ((psf = psf_allocate ()) == NULL)
	{	sf_errno = SFE_MALLOC_FAILED ;
		return	NULL ;
		} ;

	psf_init_files (psf) ;


	char *utf8name = widestr_to_utf8str (wpath) ;
	if (!utf8name)
	{	sf_errno = SFE_MALLOC_FAILED ;
		free (psf->header.ptr) ;
		free (psf) ;
		return	NULL ;
		} ;

	psf_log_printf (psf, "File : '%s' (utf-8 converted from ucs-2)\n", utf8name) ;

	copy_filename (psf, utf8name) ;
	free (utf8name) ;
	psf->file.mode = mode ;

	psf->error = psf_fopen (psf) ;

	return psf_open_file (psf, sfinfo) ;
} /* sf_wchar_open */


wchar_t *ansistr_to_widestr (const char *s)
{
	wchar_t *widestr = NULL ;

    int nRet = MultiByteToWideChar (CP_ACP, MB_PRECOMPOSED, s, -1, NULL, 0) ;
	if (nRet != 0)
	{	int cchWideChar = nRet ;
		widestr = calloc (cchWideChar, sizeof (wchar_t)) ;
		if (widestr)
		{	nRet = MultiByteToWideChar (CP_ACP, MB_PRECOMPOSED, s, -1, widestr, cchWideChar) ;
			if (nRet != cchWideChar)
            {	free(widestr) ;
                widestr = NULL ;
				}
			}
		}

    return widestr ;
}

wchar_t *utf8str_to_widestr(const char *s)
{
    wchar_t *widestr = NULL;

    int nRet = MultiByteToWideChar(CP_UTF8, 0, s, -1, NULL, 0);
    if (nRet != 0)
    {
        int cchWideChar = nRet;
        widestr = calloc(cchWideChar, sizeof(wchar_t));
        if (widestr)
        {
            nRet = MultiByteToWideChar(CP_UTF8, 0, s, -1, widestr, cchWideChar);
            if (nRet != cchWideChar)
            {
                free(widestr);
                widestr = NULL;
            }
        }
    }

    return widestr;
}

char *widestr_to_utf8str(const wchar_t *ws)
{
    char *utf8str = NULL;
    int nRet = WideCharToMultiByte(CP_UTF8, WC_ERR_INVALID_CHARS, ws, - 1, NULL, 0, NULL, NULL);
    if (nRet != 0)
    {
        int cbMultiByte = nRet;
        utf8str = calloc(cbMultiByte, sizeof(char));
        if (utf8str)
        {
            nRet = WideCharToMultiByte(CP_UTF8, WC_ERR_INVALID_CHARS, ws, - 1, utf8str, cbMultiByte, NULL, NULL);
            if (nRet != cbMultiByte)
            {
                free(utf8str);
                utf8str = NULL;
            }
        }
    }
    return utf8str;
}

char *ansistr_to_utf8str(const char *s)
{
	char *utf8str = NULL ;

    wchar_t *widestr = ansistr_to_widestr (s) ;
    if (widestr)
    {	utf8str = widestr_to_utf8str (widestr) ;
		free (widestr) ;
		widestr = NULL ;
    }

	return utf8str;
}

#endif
