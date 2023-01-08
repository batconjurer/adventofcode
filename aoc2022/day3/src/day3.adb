with Ada.Text_IO; use Ada.Text_IO;
with Ada.Strings.Unbounded;
with Ada.Containers.Vectors;

procedure Main is

   package SU renames Ada.Strings.Unbounded;

   --- The interpretation of the Rucksack for Part 1.
   type Rucksack is record
      large : SU.Unbounded_String;
      small : SU.Unbounded_String;
   end record;

   package Rucksacks is new Ada.Containers.Vectors
     (Index_Type => Natural,
      Element_Type => Rucksack);

   --- The interpretation of the Rucksack for Part 2.
   type Bag is record
      contents : SU.Unbounded_String;
   end record;

   package Bags is new Ada.Containers.Vectors
     (Index_Type => Natural,
      Element_Type => Bag);

   --- Parse the input file for Part 1.
   function ReadRucksacks (filename : String) return Rucksacks.Vector is
      package SU renames Ada.Strings.Unbounded; use SU;
      Input_File : File_Type;
      line : Unbounded_String := To_Unbounded_String ("");
      str_length : Integer := 0;
      sacks : Rucksacks.Vector;
   begin
      Open (File => Input_File, Mode => In_File, Name => filename);
      while not End_Of_File (Input_File) loop
         line := To_Unbounded_String (Get_Line (File => Input_File));
         str_length := Length (line);
         --- Split the string in half.
         sacks.Append ((
                       To_Unbounded_String (
                         To_String (line) (1 .. str_length / 2)
                        ),
                       To_Unbounded_String (
                         To_String (line) (str_length / 2 + 1 .. str_length)
                        )
                      ));
      end loop;
      Close (File => Input_File);
      return sacks;
   end ReadRucksacks;

   --- Parse the input file for Part 2.
   function ReadBags (filename : String) return Bags.Vector is
      package SU renames Ada.Strings.Unbounded; use SU;
      Input_File : File_Type;
      line : Unbounded_String := To_Unbounded_String ("");
      sacks : Bags.Vector;
   begin
      Open (File => Input_File, Mode => In_File, Name => filename);
      while not End_Of_File (Input_File) loop
         line := To_Unbounded_String (Get_Line (File => Input_File));
         sacks.Append ((contents => line));
      end loop;
      Close (File => Input_File);
      return sacks;
   end ReadBags;

   --- A method for printing out a Rucksack.
   procedure DisplayRuckSack (sack : Rucksack) is
   begin
      Put_Line ("Rucksack {");
      Put_Line ("    large => " & SU.To_String (sack.large));
      Put_Line ("    small => " & SU.To_String (sack.small));
      Put_Line ("}");
      New_Line;
   end DisplayRuckSack;

   --- Check if a string contains a given character.
   function ContainsChar (sack : SU.Unbounded_String; c : Character)
                          return Boolean
   is
      package SU renames Ada.Strings.Unbounded; use SU;
   begin
      return Index (sack, "" & c) > 0;
   end ContainsChar;

   --- Determine the sum of priorities of elements that the
   --- large and small components of a Rucksack have in common.
   function IntersectionPriority (sack : Rucksack) return Natural is
      character : constant String :=
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWYXZ";
      intersection : Natural := 0;
   begin
      --- Hacky way of iterating over all possible items.
      for J in 1 .. 52 loop
         --- Determine if a character is in both compartments.
         if ContainsChar (sack.large, character (J)) and then
           ContainsChar (sack.small, character (J))
         then
            intersection := intersection + J;
         end if;
      end loop;
      return intersection;
   end IntersectionPriority;

   --- Get the priority of the item the three sacks have in common.
   function BadgePriority (sack1, sack2, sack3 : SU.Unbounded_String) return Natural is
      character : constant String :=
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWYXZ";
      intersection : constant Natural := 0;
   begin
      --- Hacky way of iterating over all 2697possible items.
      for J in 1 .. 52 loop
         if ContainsChar (sack1, character (J)) and then
           ContainsChar (sack2, character (J)) and then
           ContainsChar (sack3, character (J))
         then
            return J;
         end if;
      end loop;

      return intersection;
   end BadgePriority;

   --- Part 1.
   procedure PartOne is
      sacks : Rucksacks.Vector;
      sum : Natural := 0;
   begin
      sacks := ReadRucksacks ("input.txt");
      for sack of sacks loop
         sum := sum + IntersectionPriority (sack);
      end loop;
      Put_Line ("Final answer: " & sum'Image);
   end PartOne;

   --- Part 2.
   procedure PartTwo is
      sacks : Bags.Vector;
      A : Natural := sacks.First_Index;
      sum : Natural := 0;
   begin
      sacks := ReadBags ("input.txt");
      while A + 2 <= sacks.Last_Index loop
         sum := sum + BadgePriority (
                                     sacks (A).contents,
                                     sacks (A + 1).contents,
                                     sacks (A + 2).contents
                                    );
         A := A + 3;
      end loop;

      Put_Line ("Final answer: " & sum'Image);
   end PartTwo;
begin
   Put_Line ("=========== Part Once ===========");
   PartOne;
   Put_Line ("=========== Part Two ===========");
   PartTwo;
end Main;
