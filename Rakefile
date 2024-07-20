require 'bundler/setup'
require 'bundler/gem_tasks'
require 'ffi'
require 'rake'
require 'rake/testtask'
require 'rb_sys/extensiontask'
require 'rspec/core/rake_task'
require 'rubygems'
require 'rubygems/package_task'

GEM_SPEC = Gem::Specification.load('bytebuffer.gemspec')

namespace :gem do

  desc 'Clean up the Gem build environment. Note: This command indiscriminately removes all .so, .dll, and .bundle files in subdirectories.'
  task :clean do
    FileUtils.rm_rf(%w(./tmp ./pkg ./target))

    case FFI::Platform::OS
    when 'linux' then FileUtils.rm_r(Dir['./**/*.so'])
    when 'darwin' then FileUtils.rm_r(Dir['./**/*.bundle'])
    when 'windows' then FileUtils.rm_r(Dir['./**/*.dll'])
    else system("rm -r ./**/*.so")
    end
  end

  desc 'Compile and install the native extension'
  task compile: %i(clean) do
    RbSys::ExtensionTask.new('bytebuffer', GEM_SPEC) do |ext|
      ext.source_pattern = "*.{rs,toml,lock,rb}"
      ext.ext_dir = "#{File.dirname(__FILE__)}/ext/bytebuffer"
      ext.cross_platform = %w[
        aarch64-linux
        arm64-darwin
        x64-mingw-ucrt
        x64-mingw32
        x86_64-darwin
        x86_64-linux
        x86_64-linux-musl
      ]

      ext.cross_compile = true
      ext.lib_dir = "lib"
    end

    Rake::Task["compile"].invoke
  end

  desc 'Apply platform patches'
  task :patch do
    working_dir = File.dirname(__FILE__)
    target = case RUBY_PLATFORM
             when /darwin/ then 'darwin'
             when /linux/ then 'linux'
             when /mswin|mingw/ then 'windows'
             end
    patch_dir = "#{working_dir}/lib/bytebuffer/patch/#{target}"
    unless !File.directory?(patch_dir) || Dir.empty?(patch_dir)
      puts "Applying platform patches for #{target}"

      FileUtils.chdir(patch_dir) do |dir|
        Dir["#{dir}/*.patch"].sort.each do |patch|
          puts "Applying patch: #{patch}"
          system("patch #{working_dir}/lib/bytebuffer.rb #{patch}")
        end
      end
    end
  end

  desc 'Revert platform patches'
  task :revert do
    working_dir = File.dirname(__FILE__)
    target = case RUBY_PLATFORM
             when /darwin/
               'darwin'
             when /linux/
               'linux'
             when /mswin|mingw/
               'windows'
             end

    FileUtils.chdir("#{working_dir}/lib/bytebuffer/patch/#{target}") do |dir|
      Dir["#{dir}/*.patch"].sort.each do |patch|
        puts "Applying patch: #{patch}"
        system("patch -R #{working_dir}/lib/bytebuffer.rb #{patch}")
      end
    end
  end

  desc 'Package the Gem package'
  task package: %i(compile gem:patch) do
    Gem::PackageTask.new(GEM_SPEC) do |pkg|
      pkg.need_tar_bz2 = true
    end

    Rake::Task['package'].invoke
  end
end

namespace :docs do
  require 'yard'

  YARD::Rake::YardocTask.new do |t|
    t.name = 'generate'
    t.files = %w(lib/**/*.rb)
  end

  task serve: %i[docs:generate] do
    Kernel.system('yard server')
  end

  task :clean do
    require 'fileutils'

    FileUtils.rm_rf('doc')
    FileUtils.rm_rf('.yardoc')
  end
end


namespace :test do
  desc 'Runs all specs in the "spec/" folder using RSpec.'
  RSpec::Core::RakeTask.new(:rspec)

  task :clean do
    require 'fileutils'

    FileUtils.rm_rf('coverage')
  end

  desc 'Runs extension tests using cargo test.'
  task :ext do
    raise "Install rustc along with Cargo before running this rake task." if !system('cargo --version') || !system('rustc --version')

    system("cargo test")
  end
end
