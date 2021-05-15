#ifndef _CONIONOVA
#define _CONIONOVA

#include <stdio.h>

#define normaltext()  printf("%c#5", 27)
#define dwidth()      printf("%c#6", 27)
#define dwdhs()       printf("%c#3", 27)
#define dwdhi()       printf("%c#4", 27)
#define allinverse()  printf("%c?5h", 155)
#define allnormal()   printf("%c?5l", 155)
#define cols132()     printf("%c?3h", 155)
#define cols80()      printf("%c?3l", 155)
#define blink()       printf("%c5;5m", 155)
#define bold()        printf("%c1;1m", 155)
#define underline()   printf("%c4;4m", 155)
#define inverse()     printf("%c7;7m", 155)
#define nobold()      printf("%c22;22m", 155)
#define nounderline() printf("%c24;24m", 155)
#define noblink()     printf("%c25;25m", 155)
#define noinverse()   printf("%c27;27m", 155)
#define normal()      printf("%c0;0m", 155)

#define clrscr() printf("%c[2J%c[%i;%iH",27,27,1,1)
#define cursor_off() printf("%c[?25l",27)
#define cursor_on() printf("%c[?25h",27)
#define key_off() printf("%c[2h",27)
#define key_on() printf("%c[2l",27)

static char Getchar() {
   char ret,cad[2];
   fflush(stdin);
   fread(cad,sizeof(char),1,stdin);
   ret=cad[0];
   while (cad[0]!='\n') fread(cad,sizeof(char),1,stdin);
   return ret;
}

static void fflushnou() {
   fflush(stdout);
   stdin->_IO_read_ptr = stdin->_IO_read_base;
   stdin->_IO_read_end = stdin->_IO_read_base;
   fflush(stdin);
}

static void gotoxy(int x,int y)
{
	printf("%c[%i;%iH",27,y,x);
}

static void negreta(int x,int y,char *s)
{
	printf("%c[%i;%iH^[#3%s\n%c[%i;%iH^[#4%s\n",27,y,x,s,27,y+1,x,s);
}

#endif
