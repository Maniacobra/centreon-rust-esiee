# Brouillon de trucs randoms testés

sudo apt update
sudo apt install protobuf-compiler

git clone https://github.com/abseil/abseil-cpp.git
cd abseil-cpp
mkdir build
cd build
make -j
cd ../../

conan profile detect --force
conan install conanfile.txt --build=missing
cmake CMakeLists.txt
