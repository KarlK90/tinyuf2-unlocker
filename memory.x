MEMORY
{
  /* TinyUF2 occupies the first 64k of FLASH on the stm32F4s */
  FLASH : ORIGIN = 0x08000000 + 64k, LENGTH = 32k
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
