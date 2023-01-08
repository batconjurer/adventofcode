with Ada.Text_IO; use Ada.Text_IO;

procedure Day4 is
   Input_File : File_Type;

   function IsSubInterval (Line : String) return Boolean is
      Last_Index : Natural := 1;
      First_Low : Natural := 0;
      First_High : Natural := 0;
      Second_Low : Natural := 0;
      Second_High : Natural := 0;
   begin
      for I in Line'Range loop
         if Line (I) = '-' then
            if First_High > 0 then
               Second_Low := Natural'Value (Line (Last_Index .. I - 1));
               Second_High := Natural'Value (Line (I + 1 .. Line'Last));
            else
               First_Low := Natural'Value (Line (Last_Index .. I - 1));
               Last_Index := I + 1;
            end if;

         elsif Line (I) = ',' then
            First_High := Natural'Value (Line (Last_Index .. I - 1));
            Last_Index := I + 1;
         end if;
      end loop;

      if Second_Low <= First_Low and then First_High <= Second_High then
         return True;
      elsif First_Low <= Second_Low and then Second_High <= First_High then
         return True;
      else
         return False;
      end if;

   end IsSubInterval;

   function IsOverlapping (Line : String) return Boolean is
      Last_Index : Natural := 1;
      First_Low : Natural := 0;
      First_High : Natural := 0;
      Second_Low : Natural := 0;
      Second_High : Natural := 0;
   begin
      for I in Line'Range loop
         if Line (I) = '-' then
            if First_High > 0 then
               Second_Low := Natural'Value (Line (Last_Index .. I - 1));
               Second_High := Natural'Value (Line (I + 1 .. Line'Last));
            else
               First_Low := Natural'Value (Line (Last_Index .. I - 1));
               Last_Index := I + 1;
            end if;

         elsif Line (I) = ',' then
            First_High := Natural'Value (Line (Last_Index .. I - 1));
            Last_Index := I + 1;
         end if;
      end loop;

      if Second_Low <= First_Low and then First_Low <= Second_High then
         return True;
      elsif Second_Low <= First_High and then First_High <= Second_High then
         return True;
      elsif First_Low <= Second_Low and then Second_Low <= First_High then
         return True;
      elsif First_Low <= Second_High and then Second_High <= First_High then
         return True;
      else
         return False;
      end if;

   end IsOverlapping;

   procedure PartOne is
      Sum : Natural := 0;
   begin
      Open (File => Input_File, Mode => In_File, Name => "input.txt");
      while not End_Of_File (Input_File) loop
         if IsSubInterval (Get_Line (Input_File)) then
            Sum := Sum + 1;
         end if;
      end loop;
      Close (Input_File);
      Put_Line ("Final Answer " & Sum'Image);
   end PartOne;

   procedure PartTwo is
      Sum : Natural := 0;
   begin
      Open (File => Input_File, Mode => In_File, Name => "input.txt");
      while not End_Of_File (Input_File) loop
         if IsOverlapping (Get_Line (Input_File)) then
            Sum := Sum + 1;
         end if;
      end loop;
      Close (Input_File);
      Put_Line ("Final Answer " & Sum'Image);
   end PartTwo;


begin
   Put_Line ("======== Part One ========");
   PartOne;
   Put_Line ("======== Part Two ========");
   PartTwo;
end Day4;
