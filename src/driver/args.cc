#include <cstdio>
#include <string>
#include <cstdint>
#include <cstring>
#include <span>
#include <clopts.hh>
using namespace command_line_options;

using options = clopts<
    multiple<positional<"file", "The file whose contents should be compiled", file<>, /*required=*/true>>,
    flag<"-v", "Enable verbose printing">,
    option<"-o", "Outfile file", std::string>,
    option<"--color", "Use colors", values<"always", "never", "default">>,
    help<>
>;

struct Args{
    bool verbose, useColors;
    const uint8_t* out_file;
    uint8_t file_paths_count;
    const uint8_t** file_paths;
};

extern "C" Args* getArgs(int argc, char** argv) {
    std::string out_file;
    std::span<command_line_options::file<>> file_paths;
    bool verbose, useColors;
    auto opts = options::parse(argc, argv);
    file_paths = opts.get<"file">();
    out_file = opts.get_or<"-o">("a.out");
    verbose = opts.get<"-v">();
    std::string colorOpt = opts.get_or<"--color">("always");
    useColors = colorOpt == "always";
    if(file_paths.empty()){
        std::printf("ERROR: No file paths provided\n");
        std::exit(1);
    }
    Args* args = reinterpret_cast<Args*>(malloc(sizeof(Args)));
    args->verbose = verbose;
    args->useColors = useColors;
    args->out_file = new uint8_t[out_file.size() + 1];
    args->file_paths = new const uint8_t*[file_paths.size()];
    args->file_paths_count = 0;
    for (command_line_options::file<> file : file_paths) {
        const uint8_t* buffer = new uint8_t[file.path.string().size()+1];
        std::snprintf((char*)buffer, file.path.string().size()+1, "%s", file.path.string().c_str());
        args->file_paths[args->file_paths_count++] = buffer;
    }
    std::memcpy((void*)args->out_file, out_file.c_str(), out_file.size());
    return args;
}
